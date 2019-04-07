extern crate cave;
extern crate glutin_window;
extern crate graphics;
extern crate piston;
extern crate specs;

use cave::app::{Application, Settings};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::default()
        .title("Visual Test")
        .window_size(800, 600)
        .fps(60)
        .ups(60);

    let mut app = Application::new(settings);
    app.run()?;

    Ok(())
}
