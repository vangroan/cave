
use std::iter::IntoIterator;
use std::marker::PhantomData;

use na::{Vector3};

// TODO: Make 3D
const OFFSETS : [(i32, i32, i32); 8] = [
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
];

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

    pub fn neighbours(&self, pos: &GridPosition) -> [Option<GridPosition>; 8] {
        let mut n = [None, None, None, None, None, None, None, None];

        for i in 0..8 {
            let offset = OFFSETS[i];
            let v = Vector3::<i32>::new(
                pos.0.x + offset.0,
                pos.0.y + offset.1,
                pos.0.z + offset.2,
            );
            let new_pos = GridPosition(v);
            n[i] = if self.in_bounds(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        }

        n
    }

    fn in_bounds(&self, pos: &GridPosition) -> bool {
        pos.x() >= 0 && pos.x() < self.size.x as i32 && pos.y() >= 0 && pos.y() < self.size.y as i32
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridPosition(Vector3<i32>);

impl GridPosition {
    pub fn new(x: i32, y: i32, z: i32) -> GridPosition {
        GridPosition(Vector3::new(x, y, z))
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
    }

    pub fn z(&self) -> i32 {
        self.0.z
    }
}
