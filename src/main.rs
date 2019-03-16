extern crate glutin_window;
extern crate graphics;
extern crate nalgebra as na;
extern crate num_traits as nt;
extern crate opengl_graphics;
extern crate piston;
extern crate shred;
extern crate specs;
extern crate sprite;
#[macro_use]
extern crate specs_derive;
extern crate rayon;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use specs::{DispatcherBuilder, World};
use sprite::{Scene, Sprite};

use std::path::PathBuf;
use std::rc::Rc;

mod grid;
mod isometric;
mod option;
mod pathfinder;
mod tilemap;

use grid::Grid;
use isometric::Isometric;
use pathfinder::{PathRequests, PathResults, Pathfinder, PathfindingSystem};
use tilemap::{TileObj, Tilemap};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    world: World,
    scene: Scene<Texture>,
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

        let App { gl, scene, .. } = self;

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y);
            //.rot_rad(rotation)
            //.trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            // rectangle(RED, square, transform, gl);
            scene.draw(transform, gl);
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

    // Setup ECS
    let mut world = World::new();
    world.add_resource(Grid::with_size(128, 128, 128));
    world.add_resource(Tilemap::with_size(128, 128, 128));
    world.add_resource(Pathfinder::new());
    world.add_resource(PathRequests::new());
    world.add_resource(PathResults::new());
    world.register::<TileObj>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PathfindingSystem, "pathfinder", &[])
        .build();

    let sprite_path = PathBuf::from("resources/greybox.png");
    let sprite_settings = TextureSettings::new();
    let mut scene = Scene::<Texture>::new();
    let tex = Rc::new(Texture::from_path(sprite_path, &sprite_settings).unwrap());

    for x in 0..10 {
        for y in 0..10 {
            for z in (0..10).rev() {
                const s: f64 = 80.;
                const d: f64 = 50.;
                let pos = Isometric::cart_to_iso(na::Vector3::<f64>::new(
                    x as f64 * s,
                    y as f64 * s,
                    z as f64 * d,
                ));
                let mut sprite = Sprite::from_texture(tex.clone());
                sprite.set_position(pos.x, pos.y + pos.z);
                scene.add_child(sprite);
            }
        }
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        world,
        scene,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
            dispatcher.dispatch(&mut app.world.res);
            app.world.maintain();
        }
    }
}
