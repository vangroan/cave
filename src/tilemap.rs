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

    #[inline(always)]
    pub fn set_tile(&mut self, pos: &GridPosition, tile: Tile) {
        self.data[grid_index(&self.size, pos)] = tile;
    }

    #[inline(always)]
    pub fn tile(&self, pos: &GridPosition) -> Option<&Tile> {
        self.data.get(grid_index(&self.size, pos))
    }

    pub fn is_passable(&self, pos: &GridPosition) -> bool {
        self.data
            .get(grid_index(&self.size, pos))
            .map(|tile| tile != &Tile::GreyBlock)
            .unwrap_or(false)
    }
}

impl Default for Tilemap {
    fn default() -> Tilemap {
        Tilemap::with_size(16, 16, 16)
    }
}

#[derive(Eq, PartialEq)]
pub enum Tile {
    Empty,
    GreyBlock,
    Ladder,
}

/// Marks an entity as locked to the tilemap grid
#[derive(Component)]
pub struct TileObj {
    pos: GridPosition,
}
