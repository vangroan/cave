use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::error::Error;

use super::settings::Settings;

pub type AppResult = Result<(), Box<dyn Error>>;

pub struct Application {
    settings: Settings,
}

impl Application {
    pub fn new(settings: Settings) -> Self {
        Application { settings }
    }

    pub fn run(&self) -> AppResult {
        let Application { settings } = self;

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
        while let Some(e) = events.next(&mut window) {}

        Ok(())
    }
}
