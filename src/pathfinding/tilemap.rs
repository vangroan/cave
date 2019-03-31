//! Strategies to enforce pathfinding rules based on the Tilemap terrain

use super::cost::*;
use super::locomotion::*;
use crate::grid::GridPosition;
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
    fn is_passable(&self, _source: &GridPosition, target: &GridPosition) -> Cost {
        // TODO: Diagonal costs
        if self.tilemap.is_passable(target) {
            Cost::Passable(10)
        } else {
            Cost::Blocked
        }
    }
}

pub struct TilemapLocomotion<'a> {
    tilemap: &'a Tilemap,
}

impl<'a> TilemapLocomotion<'a> {
    pub fn new(tilemap: &'a Tilemap) -> Self {
        TilemapLocomotion { tilemap }
    }
}

impl<'a> LocomotionStrategy for TilemapLocomotion<'a> {
    #[inline(always)]
    fn is_passable(&self, _source: &GridPosition, _target: &GridPosition) -> bool {
        // TODO: Check if the entity can actually move there
        true
    }
}
