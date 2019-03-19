use std::ops::Add;

use na::Vector3;
use specs::prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Position(Vector3<f64>);

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position(Vector3::new(x, y, z))
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.0.x
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.0.z
    }

    #[inline(always)]
    pub fn to_vector(&self) -> &na::Vector3<f64> {
        &self.0
    }
}

impl Add<&Position> for &Position {
    type Output = Position;
    fn add(self, rhs: &Position) -> Position {
        Position(self.0 + rhs.0)
    }
}
