extern crate cave;
extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate specs;

use cave::app::{Application, Settings};
use cave::scene::Scene;
use cave::view::DepthGraph;
use specs::{DispatcherBuilder, World};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut world = World::new();
    world.add_resource(DepthGraph::new());

    // Update
    let update = DispatcherBuilder::new().build();

    // Render
    let render = DispatcherBuilder::new().build();

    // Scene
    let scene = Scene::new(world, update, render);

    // Settings
    let settings = Settings::default()
        .title("Visual Test")
        .window_size(800, 600)
        .fps(60)
        .ups(60);

    // Application
    let mut app = Application::with_scene(settings, scene);
    app.run()?;

    Ok(())
}
