
use na::Vector3;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_index() {
        let size = Vector3::<u32>::new(10, 10, 10);
        assert_eq!(432, grid_index(&size, &GridPosition::new(2, 3, 4)));
        assert_eq!(999, grid_index(&size, &GridPosition::new(9, 9, 9)));
    }
}