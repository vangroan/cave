
use na::{Vector3};

// TODO: Make 3D
const OFFSETS : [Vector3<i32>; 8] = [
    Vector3::new(-1, -1, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(1, -1, 0),
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(-1, 1, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(1, 1, 0),
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

    pub fn neighbours(&self, pos: &GridPosition) -> Neighbours {
        let mut n = Neighbours([None, None, None, None, None, None, None, None]);

        for i in 0..8 {
            let v = GridPosition(pos.0 + OFFSETS[i]);
            n.0[i] = if self.in_bounds(&v) {
                Some(v)
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

pub struct Neighbours([Option<GridPosition>; 8]);

impl Neighbours {
    pub fn into_iter<'a>(self) -> Iter<'a>{
        Iter::new(self.0.into_iter().filter_map(|maybe_pos| *maybe_pos))
    }
}

pub struct Iter<'i> {
    // TODO: Remove heap alloc
    inner: &'i mut Iterator<Item = GridPosition>,
}

impl<'i> Iter<'i> {
    fn new<I>(inner: I) -> Self
    where
        I: Iterator<Item = GridPosition> + 'i
    {
        Iter {
            inner: &mut inner
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = GridPosition;

    fn next(&mut self) -> Option<GridPosition> {
        self.inner.next()
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
