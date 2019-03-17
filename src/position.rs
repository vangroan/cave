
use na::Vector3;
use specs::prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Position(Vector3<f64>);

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position(Vector3::new(x, y, z))
    }
}
