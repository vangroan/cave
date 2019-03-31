//! Helper strategies that basically do nothing
//!
//! For use in tests and benchmarks

use super::cost::*;
use super::locomotion::*;
use crate::grid::GridPosition;

/// Always returns a passable cost
pub struct NoOpCost;

impl CostStrategy for NoOpCost {
    #[inline]
    fn is_passable(&self, _source: &GridPosition, _target: &GridPosition) -> Cost {
        Cost::Passable(10)
    }
}

/// Always returns as passable
pub struct NoOpLocomotion;

impl LocomotionStrategy for NoOpLocomotion {
    #[inline]
    fn is_passable(
        &self,
        _locomotion: &Locomotion,
        _source: &GridPosition,
        _target: &GridPosition,
    ) -> bool {
        true
    }
}
