extern crate glutin_window;
extern crate graphics;
extern crate nalgebra as na;
extern crate num_traits as nt;
extern crate opengl_graphics;
extern crate piston;
extern crate shred;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate rayon;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use specs::prelude::*;

use std::path::PathBuf;
use std::sync::Arc;

mod actor;
mod camera;
mod grid;
mod isometric;
mod option;
mod pathfinder;
mod pigeon;
mod position;
mod sort;
mod sprite;
mod tilemap;

use actor::Actor;
use camera::IsometricCamera;
use grid::Grid;
use isometric::Isometric;
use pathfinder::{Pather, Pathfinder, PathfindingSystem};
use position::Position;
use sort::{DepthBuffer, IsometricSorter};
use sprite::{OnRender, Sprite, SpriteRenderer};
use tilemap::{TileObj, Tilemap};

const TILE_WIDTH_2D: f64 = 80.;
const TILE_DEPTH_2D: f64 = 50.;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("cave", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Map Size
    const MAP_WIDTH: u32 = 128;
    const MAP_HEIGHT: u32 = 128;
    const MAP_DEPTH: u32 = 128;

    // Setup ECS
    let mut world = World::new();
    world.add_resource(Grid::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH));
    world.add_resource(Tilemap::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH));
    world.add_resource(Pathfinder::new());
    world.add_resource(DepthBuffer::new());
    world.register::<Actor>();
    world.register::<IsometricCamera>();
    world.register::<TileObj>();
    world.register::<Sprite<Texture>>();
    world.register::<Pather>();
    world.register::<Position>();

    let mut update_dispatcher = DispatcherBuilder::new()
        .with(PathfindingSystem, "pathfinder", &[])
        .with(
            IsometricSorter::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH),
            "isometric_sorter",
            &[],
        )
        .build();
    let mut render_dispatcher = DispatcherBuilder::new()
        .with_thread_local(SpriteRenderer::from_graphics(GlGraphics::new(opengl)))
        .build();

    let sprite_settings = TextureSettings::new();
    let block_tex = Arc::new(Texture::from_path(PathBuf::from("resources/greybox.png"), &sprite_settings).unwrap());
    let man_tex = Arc::new(Texture::from_path(PathBuf::from("resources/blueman.png"), &sprite_settings).unwrap());

    // Build Camera
    world.create_entity()
        .with(IsometricCamera::new(true))
        .with(Position::new(0., 0., 10.))
        .build();

    // Build blocks
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                // if x + y + z > 7 {
                //     continue;
                // }
                // if (x + y + z) % 2 == 0 {
                //     continue;
                // }
                if x >= 5 && y >= 5 && z >= 5 {
                    continue;
                }
                let pos = Isometric::cart_to_iso(&na::Vector3::<f64>::new(
                    x as f64 * TILE_WIDTH_2D,
                    y as f64 * TILE_WIDTH_2D,
                    z as f64 * TILE_DEPTH_2D,
                ));
                let mut sprite = Sprite::from_texture(block_tex.clone());
                sprite.set_position(pos.x, pos.y - pos.z);
                sprite.set_anchor(0.5, 70. / 90.);

                // Lower blocks are darker
                let c = 0.8 + (z as f32 / 50.);
                sprite.set_color([c, c, c, 1.0]);

                world
                    .create_entity()
                    .with(Position::new(x as f64, y as f64, z as f64))
                    .with(sprite)
                    .build();
            }
        }
    }

    // Build actors
    for x in 0..10 {
        for y in 0..10 {
            const HALF_TILE_3D : f64 = 0.5;
            let z = 11;
            let pos = Isometric::cart_to_iso(&na::Vector3::<f64>::new(
                (x as f64 + HALF_TILE_3D) * TILE_WIDTH_2D,
                (y as f64 + HALF_TILE_3D) * TILE_WIDTH_2D,
                z as f64 * TILE_DEPTH_2D,
            ));
            if (x + y) % 5 != 0 {
                continue;
            }

            let mut sprite = Sprite::from_texture(man_tex.clone());
            sprite.set_position(pos.x, pos.y - pos.z);
            sprite.set_anchor(0.5, 1.0);

            world
                .create_entity()
                .with(Position::new(x as f64, y as f64, z as f64))
                .with(sprite)
                .with(Actor::new())
                .with(Pather::new())
                .build();
        }
    }

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            // app.render(&r);
            world.add_resource(OnRender::new(r));
            update_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }

        if let Some(u) = e.update_args() {
            // app.update(&u);
            render_dispatcher.dispatch(&mut world.res);
            // app.world.maintain();
            world.maintain();
        }
    }
}
