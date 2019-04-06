extern crate fps_counter;
extern crate glutin_window;
extern crate graphics;
extern crate nalgebra as na;
extern crate num_traits as nt;
extern crate opengl_graphics;
extern crate petgraph;
extern crate piston;
extern crate rayon;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate threadpool;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use specs::prelude::*;
use specs::Entity;

use std::path::PathBuf;
use std::sync::Arc;

mod actor;
mod common;
mod depthsort;
mod grid;
mod isometric;
mod option;
mod pathfinding;
mod pigeon;
mod position;
mod settings;
mod sprite;
mod tilemap;
mod view;

use actor::{Actor, WalkerSystem};
use common::DeltaTime;
use depthsort::{DepthBuffer, IsometricSorter};
use grid::{Grid, GridPosition};
use pathfinding::{
    components::Pather, systems::PathfindingSystem, AStar, Locomotion, CLIMB_LADDERS, GROUND_WALK,
};
use position::Position;
use sprite::{OnRender, Sprite, SpriteRenderer};
use tilemap::{Tile, TileObj, Tilemap};
use view::{components::IsometricCamera, CutMode, ViewCutMode};

fn create_block(
    world: &mut World,
    tile: Tile,
    block_tex: Arc<Texture>,
    grid_pos: &GridPosition,
) -> Entity {
    world.write_resource::<Tilemap>().set_tile(&grid_pos, tile);

    let mut sprite = Sprite::from_texture(block_tex);
    sprite.set_anchor(0.5, 70. / 90.);

    // Lower blocks are darker
    let c = 0.8 + (grid_pos.z() as f32 / 50.);
    sprite.set_color([c, c, c, 1.0]);

    world
        .create_entity()
        .with(Position::new(
            grid_pos.x() as f64,
            grid_pos.y() as f64,
            grid_pos.z() as f64,
        ))
        .with(sprite)
        .build()
}

fn create_actor(
    world: &mut World,
    man_tex: Arc<Texture>,
    grid_pos: &GridPosition,
    path_to: &GridPosition,
) -> Entity {
    let mut sprite = Sprite::from_texture(man_tex.clone());
    sprite.set_anchor(0.5, 0.9);

    world
        .create_entity()
        .with(Position::new(
            grid_pos.x() as f64,
            grid_pos.y() as f64,
            grid_pos.z() as f64,
        ))
        .with(sprite)
        .with(Actor::with_speed(1.0))
        .with(Pather::with_request(grid_pos.clone(), path_to.clone()))
        .with(Locomotion::new(&[GROUND_WALK, CLIMB_LADDERS]))
        .build()
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
    const MAP_WIDTH: u32 = 16;
    const MAP_HEIGHT: u32 = 16;
    const MAP_DEPTH: u32 = 16;

    // Setup ECS
    let mut world = World::new();
    world.add_resource(Grid::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH));
    world.add_resource(Tilemap::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH));
    world.add_resource(AStar::new());
    world.add_resource(DepthBuffer::new());
    world.add_resource(ViewCutMode::default());
    world.register::<Actor>();
    world.register::<IsometricCamera>();
    world.register::<Locomotion>();
    world.register::<TileObj>();
    world.register::<Sprite<Texture>>();
    world.register::<Pather>();
    world.register::<Position>();
    world.register::<GridPosition>();

    let mut update_dispatcher = DispatcherBuilder::new()
        .with(PathfindingSystem::new(), "pathfinder", &[])
        .with(
            IsometricSorter::with_size(MAP_WIDTH, MAP_HEIGHT, MAP_DEPTH),
            "isometric_sorter",
            &[],
        )
        .with(WalkerSystem::new(), "walker", &[])
        .build();
    let mut render_dispatcher = DispatcherBuilder::new()
        .with_thread_local(SpriteRenderer::from_graphics(GlGraphics::new(opengl)))
        .build();

    let sprite_settings = TextureSettings::new();
    let block_tex = Arc::new(
        Texture::from_path(PathBuf::from("resources/greybox.png"), &sprite_settings).unwrap(),
    );
    let man_tex = Arc::new(
        Texture::from_path(PathBuf::from("resources/blueman.png"), &sprite_settings).unwrap(),
    );
    let ladder_tex = Arc::new(
        Texture::from_path(PathBuf::from("resources/ladder.png"), &sprite_settings).unwrap(),
    );

    // Build Camera
    world
        .create_entity()
        .with(IsometricCamera::new(true))
        .with(Position::new(0., 0., 9.))
        .build();

    // Build blocks
    for x in 0..MAP_WIDTH as i32 {
        for y in 0..MAP_HEIGHT as i32 {
            for z in 0..MAP_DEPTH as i32 {
                // if x + y + z > 7 {
                //     continue;
                // }
                // if (x + y + z) % 2 == 0 {
                //     continue;
                // }
                if x >= 5 && y >= 5 && z >= 5 {
                    continue;
                }
                if z == 9 && (x % 2 == 0 || y % 2 == 0) {
                    continue;
                }
                // if x != y || y != z || x != z {
                //     continue;
                // }
                let grid_pos = GridPosition::new(x, y, z);
                create_block(&mut world, Tile::GreyBlock, block_tex.clone(), &grid_pos);
            }
        }
    }

    // Build Ladders
    create_block(
        &mut world,
        Tile::Ladder,
        ladder_tex.clone(),
        &GridPosition::new(5, 5, 8),
    );
    create_block(
        &mut world,
        Tile::Ladder,
        ladder_tex.clone(),
        &GridPosition::new(5, 5, 7),
    );
    create_block(
        &mut world,
        Tile::Ladder,
        ladder_tex.clone(),
        &GridPosition::new(5, 5, 6),
    );
    create_block(
        &mut world,
        Tile::Ladder,
        ladder_tex.clone(),
        &GridPosition::new(5, 5, 5),
    );

    // create_block(&mut world, block_tex.clone(), &GridPosition::new(0, 0, 0));
    // create_block(&mut world, block_tex.clone(), &GridPosition::new(1, 1, 1));
    // create_block(&mut world, block_tex.clone(), &GridPosition::new(2, 2, 2));
    // create_block(&mut world, block_tex.clone(), &GridPosition::new(2, 2, 1));

    // Build actors
    // for x in 0..1 {
    //     for y in 0..1 {
    //         if (x + y) % 2 != 0 {
    //             continue;
    //         }
    //         if x >= 5 && y >= 5 {
    //             continue;
    //         }

    //         let z = 9;
    //         create_actor(&mut world, man_tex.clone(), &GridPosition::new(x, y, z), &GridPosition::new(9, 9, 5));
    //     }
    // }

    // Build Actor
    create_actor(&mut world, man_tex.clone(), &GridPosition::new(9, 9, 5), &GridPosition::new(4, 4, 9));

    let settings = EventSettings::new().max_fps(60).ups(60);

    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.release_args() {
            use specs::Join;

            // TODO: Move input handling to systems
            let entities = world.entities();
            let cameras = world.read_storage::<IsometricCamera>();
            let mut positions = world.write_storage::<Position>();
            let mut view_cut = world.write_resource::<ViewCutMode>();
            let maybe_camera = (&entities, &cameras, &positions)
                .join()
                .find(|(_entity, camera, _position)| camera.is_current());
            if let Some((entity, _camera, pos)) = maybe_camera {
                match key {
                    Key::Up => {
                        positions
                            .insert(entity, pos + &Position::new(-1., -1., 0.))
                            .unwrap();
                    }
                    Key::Down => {
                        positions
                            .insert(entity, pos + &Position::new(1., 1., 0.))
                            .unwrap();
                    }
                    Key::Left => {
                        positions
                            .insert(entity, pos + &Position::new(-1., 1., 0.))
                            .unwrap();
                    }
                    Key::Right => {
                        positions
                            .insert(entity, pos + &Position::new(1., -1., 0.))
                            .unwrap();
                    }
                    Key::Q => {
                        positions
                            .insert(entity, pos + &Position::new(0., 0., 1.))
                            .unwrap();
                    }
                    Key::A => {
                        positions
                            .insert(entity, pos + &Position::new(0., 0., -1.))
                            .unwrap();
                    }
                    Key::D1 => {
                        println!("View from Top");
                        view_cut.set_mode(CutMode::Top);
                    }
                    Key::D2 => {
                        println!("View from Left");
                        view_cut.set_mode(CutMode::Left);
                    }
                    Key::D3 => {
                        println!("View from Right");
                        view_cut.set_mode(CutMode::Right);
                    }
                    _ => {}
                }
            }
        }

        if let Some(u) = e.update_args() {
            world.add_resource(DeltaTime(u.dt));
            // pathfinding_system.run_now(&world.res);
            // walker_system.run_now(&world.res);
            // isometric_sorter_system.run_now(&world.res);
            update_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }

        if let Some(r) = e.render_args() {
            // app.render(&r);
            world.add_resource(OnRender::new(r));
            // sprite_render_system.run_now(&world.res);
            render_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }
    }
}
