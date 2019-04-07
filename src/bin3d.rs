//! A rewrite using true 3D graphics

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::GlRequest;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Square Toy".to_string())
        .with_dimensions((800, 600).into());
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_vsync(true);

    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop)
            .expect("Failed to initialize gfx_window_glutin");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;
    while running {
        events_loop.poll_events(|ev| {
            use glutin::Event::*;

            match ev {
                WindowEvent { event, .. } => {
                    use glutin::WindowEvent::*;

                    match event {
                        KeyboardInput {
                            input:
                                glutin::KeyboardInput {
                                    virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        }
                        | CloseRequested => running = false,
                        Resized(_) => {
                            gfx_glutin::update_views(&window, &mut main_color, &mut main_depth);
                        }
                        _ => (),
                    }
                }
                DeviceEvent { .. } => {}
                _ => {}
            }
        });

        encoder.clear(&main_color, BLACK);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
