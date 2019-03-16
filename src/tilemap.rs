use crate::grid::{grid_index, GridPosition};
use specs::prelude::*;

pub struct Tilemap {
    size: na::Vector3<u32>,
    data: Vec<Tile>,
}

impl Tilemap {
    pub fn with_size(x: u32, y: u32, z: u32) -> Tilemap {
        Tilemap {
            size: na::Vector3::new(x, y, z),
            data: (0..(x * y * z)).map(|_| Tile::Empty).collect(),
        }
    }

    pub fn set_tile(&mut self, pos: &GridPosition, tile: Tile) {
        self.data[grid_index(&self.size, pos)] = tile;
    }

    pub fn tile(&self, pos: &GridPosition) -> Option<&Tile> {
        self.data.get(grid_index(&self.size, pos))
    }
}

pub enum Tile {
    Empty,
    GreyBlock,
}

/// Marks an entity as locked to the tilemap grid
#[derive(Component)]
pub struct TileObj {
    pos: GridPosition,
}
