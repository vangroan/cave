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

mod grid;
mod isometric;
mod option;
mod pathfinder;
mod pigeon;
mod position;
mod sort;
mod sprite;
mod tilemap;

use grid::Grid;
use isometric::Isometric;
use pathfinder::{PathRequests, PathResults, Pathfinder, PathfindingSystem};
use position::Position;
use sort::{DepthBuffer, IsometricSorter};
use sprite::{OnRender, Sprite, SpriteRenderer};
use tilemap::{TileObj, Tilemap};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    world: World,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.width / 4.0, args.height / 4.0);

        let App { gl, .. } = self;

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y);
            // .rot_rad(rotation)
            // .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            // rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

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
    world.add_resource(PathRequests::new());
    world.add_resource(PathResults::new());
    world.add_resource(DepthBuffer::new());
    world.register::<TileObj>();
    world.register::<Sprite<Texture>>();
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

    let sprite_path = PathBuf::from("resources/greybox.png");
    let sprite_settings = TextureSettings::new();
    // let mut scene = sprite2::Scene::<Texture>::new();
    let tex = Arc::new(Texture::from_path(sprite_path, &sprite_settings).unwrap());

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
                const S: f64 = 80.;
                const D: f64 = 50.;
                let pos = Isometric::cart_to_iso(na::Vector3::<f64>::new(
                    x as f64 * S,
                    y as f64 * S,
                    z as f64 * D,
                ));
                let mut sprite = Sprite::from_texture(tex.clone());
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

    // Create a new game and run it.
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     rotation: 0.0,
    //     world,
    // };

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
