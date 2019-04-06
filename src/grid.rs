use na::Vector3;
use specs::prelude::*;

pub const NEIGHBOUR_COUNT_2D: usize = 8;
pub const NEIGHBOUR_COUNT_3D: usize = 26;

const OFFSETS: [(i32, i32, i32); NEIGHBOUR_COUNT_2D] = [
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
];

const OFFSETS_3D: [(i32, i32, i32); NEIGHBOUR_COUNT_3D] = [
    // Middle Level
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
    // Upper Level
    (-1, -1, 1),
    (0, -1, 1),
    (1, -1, 1),
    (-1, 0, 1),
    (0, 0, 1),
    (1, 0, 1),
    (-1, 1, 1),
    (0, 1, 1),
    (1, 1, 1),
    // Lower Level
    (-1, -1, -1),
    (0, -1, -1),
    (1, -1, -1),
    (-1, 0, -1),
    (0, 0, -1),
    (1, 0, -1),
    (-1, 1, -1),
    (0, 1, -1),
    (1, 1, -1),
];

/// Return a one dimensional array or vector index given
/// a 3D cube size and a grid position.
#[inline]
pub fn grid_index(size: &Vector3<u32>, pos: &GridPosition) -> usize {
    let x = pos.x() as u32;
    let y = pos.y() as u32;
    let z = pos.z() as u32;
    (x + (y * size.x) + (z * (size.x * size.y))) as usize
}

/// Return a one dimensional array or vector index given
/// a 3D cube size and a grid position.
///
/// Similar to `grid_index` but takes unsigned ints
#[inline]
pub fn grid_index_u(size: &Vector3<u32>, pos: &(u32, u32, u32)) -> usize {
    let x = pos.0;
    let y = pos.1;
    let z = pos.2;
    (x + (y * size.x) + (z * (size.x * size.y))) as usize
}

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

    pub fn size(&self) -> (u32, u32, u32) {
        (self.size.x, self.size.y, self.size.z)
    }

    pub fn neighbours(&self, pos: &GridPosition) -> [Option<GridPosition>; NEIGHBOUR_COUNT_2D] {
        let mut n = [None, None, None, None, None, None, None, None];

        for i in 0..8 {
            let offset = OFFSETS[i];
            let v = Vector3::<i32>::new(pos.0.x + offset.0, pos.0.y + offset.1, pos.0.z + offset.2);
            let new_pos = GridPosition(v);
            n[i] = if self.in_bounds(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        }

        n
    }

    pub fn neighbours_3d(&self, pos: &GridPosition) -> [Option<GridPosition>; NEIGHBOUR_COUNT_3D] {
        let mut n = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None,
        ];

        for i in 0..26 {
            let offset = OFFSETS_3D[i];
            let v = Vector3::<i32>::new(pos.0.x + offset.0, pos.0.y + offset.1, pos.0.z + offset.2);
            let new_pos = GridPosition(v);
            n[i] = if self.in_bounds(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        }

        n
    }

    #[inline(always)]
    pub fn in_bounds(&self, pos: &GridPosition) -> bool {
        pos.x() >= 0
            && pos.x() < self.size.x as i32
            && pos.y() >= 0
            && pos.y() < self.size.y as i32
            && pos.z() >= 0
            && pos.z() < self.size.z as i32
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            size: na::Vector3::new(16, 16, 16),
        }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug)]
#[storage(DenseVecStorage)]
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

    pub fn vector(&self) -> &na::Vector3<i32> {
        &self.0
    }

    /// Tests for diagonality on the x, y plane
    pub fn is_diagonal_2d(&self, rhs: &GridPosition) -> bool {
        let x = rhs.0.x - self.0.x;
        let y = rhs.0.y - self.0.y;
        x != 0 && y != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_index() {
        let size = Vector3::<u32>::new(10, 10, 10);
        assert_eq!(432, grid_index(&size, &GridPosition::new(2, 3, 4)));
        assert_eq!(999, grid_index(&size, &GridPosition::new(9, 9, 9)));
    }

    #[test]
    fn test_diagonal_2d() {
        {
            let a = GridPosition::new(0, 1, 0);
            let b = GridPosition::new(1, 2, 0);
            assert!(a.is_diagonal_2d(&b));
        }

        {
            let a = GridPosition::new(1, 2, 0);
            let b = GridPosition::new(1, 1, 0);
            assert!(!a.is_diagonal_2d(&b));
        }

        {
            let a = GridPosition::new(-1, -2, 0);
            let b = GridPosition::new(-1, -1, 0);
            assert!(!a.is_diagonal_2d(&b));
        }
    }
}
