use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::error::Error;

use crate::common::components::{DeltaTime, OnRender};
use crate::scene::*;

use super::settings::Settings;

pub type AppResult = Result<(), Box<dyn Error>>;

pub struct Application<'a, 'b> {
    settings: Settings,
    scene: Option<Scene<'a, 'b>>,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new(settings: Settings) -> Self {
        Application {
            settings,
            scene: None,
        }
    }

    pub fn with_scene(settings: Settings, scene: Scene<'a, 'b>) -> Self {
        Application {
            settings,
            scene: Some(scene),
        }
    }

    pub fn run(&mut self) -> AppResult {
        let &mut Application {
            ref settings,
            ref mut scene,
        } = self;

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let mut window: Window = WindowSettings::new(
            settings.title,
            [settings.window_width, settings.window_height],
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()?;

        let mut events = Events::new(EventSettings::new().max_fps(settings.fps).ups(settings.ups));
        while let Some(e) = events.next(&mut window) {
            if let Some(u) = e.update_args() {
                if let Some(s) = scene {
                    s.world.add_resource(DeltaTime(u.dt));
                    s.update_dispatcher.dispatch(&mut s.world.res);
                    s.world.maintain();
                }
            }

            if let Some(r) = e.render_args() {
                if let Some(s) = scene {
                    s.world.add_resource(OnRender::new(r));
                    s.render_dispatcher.dispatch(&mut s.world.res);
                    s.world.maintain();
                }
            }
        }

        Ok(())
    }
}
