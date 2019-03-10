
use na::{Vector3};

/// 3-Dimensional Grid
pub struct Grid {
    size: Vector3<u32>,
}

impl Grid {
    pub fn with_size(x: u32, y: u32, z: u32) -> Grid {
        Grid {
            size: Vector3::new(x, y, z),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GridPosition(Vector3<i32>);

impl GridPosition {
    pub fn new(x: i32, y: i32, z: i32) -> GridPosition {
        GridPosition(Vector3::new(x, y, z))
    }
}
