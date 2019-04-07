extern crate cgmath;
extern crate three;

use cgmath::{Deg, Euler, Quaternion};
use three::Object;

fn main() {
    let title = "Getting started with three-rs";
    let mut window = three::Window::new(title);

    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
    let material = three::material::Basic {
        color: 0xCCCCCC,
        map: None,
    };
    let mesh = window.factory.mesh(geometry, material);

    window.scene.add(&mesh);
    window.scene.background = three::Background::Color(0x222222);

    let rot1 = Quaternion::from(Euler {
        x: Deg(0.0),
        y: Deg(45.0),
        z: Deg(0.0),
    });
    let rot2 = Quaternion::from(Euler {
        x: Deg(30.0),
        y: Deg(0.0),
        z: Deg(0.0),
    });

    let center = [0.0, 0.0];
    let yextent = 4.0;
    let zrange = -1.0..1.0;
    let camera = window.factory.orthographic_camera(center, yextent, zrange);
    camera.set_orientation(rot1 * rot2);

    while window.update() {
        window.render(&camera);
    }
}
