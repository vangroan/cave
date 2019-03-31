//! Strategies to enforce pathfinding rules based on the Tilemap terrain

use super::cost::*;
use super::locomotion::*;
use crate::grid::*;
use crate::tilemap::Tilemap;

pub struct TilemapCost<'a> {
    tilemap: &'a Tilemap,
}

impl<'a> TilemapCost<'a> {
    pub fn new(tilemap: &'a Tilemap) -> Self {
        TilemapCost { tilemap }
    }
}

impl<'a> CostStrategy for TilemapCost<'a> {
    #[inline(always)]
    fn is_passable(&self, source: &GridPosition, target: &GridPosition) -> Cost {
        // TODO: Diagonal costs
        if self.tilemap.is_passable(target) {
            if source.is_diagonal_2d(target) {
                Cost::Passable(14)
            } else {
                Cost::Passable(10)
            }
        } else {
            Cost::Blocked
        }
    }
}

pub struct TilemapLocomotion<'a> {
    tilemap: &'a Tilemap,
    grid: &'a Grid,
}

impl<'a> TilemapLocomotion<'a> {
    pub fn new(tilemap: &'a Tilemap, grid: &'a Grid) -> Self {
        TilemapLocomotion { tilemap, grid }
    }
}

impl<'a> LocomotionStrategy for TilemapLocomotion<'a> {
    #[inline(always)]
    fn is_passable(
        &self,
        locomotion: &Locomotion,
        _source: &GridPosition,
        target: &GridPosition,
    ) -> bool {
        if locomotion.has_method(GROUND_WALK) {
            // "I need solid ground to stand on"
            let beneath = GridPosition::new(target.x(), target.y(), target.z() - 1);

            if !self.grid.in_bounds(&beneath) {
                // Bottom, or edge, of world
                return false;
            }

            if !self.tilemap.is_passable(&beneath) {
                // "I can stand on this"
                return true;
            }
        }

        false
    }
}
