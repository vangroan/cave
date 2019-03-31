use crate::grid::GridPosition;
use std::cmp::Ordering;

pub struct PathNode {
    pub pos: GridPosition,
    /// distance from start
    pub g: u32,
    /// heuristic
    pub h: u32,
    /// total cost
    pub cost: u32,
}

/// Wrapper for `GridPosition` and cost to allow for min-heap compare
#[derive(Eq, PartialEq)]
pub struct PathNodePos(pub GridPosition, pub u32);

impl Ord for PathNodePos {
    fn cmp(&self, other: &PathNodePos) -> Ordering {
        // Note that this is backwards to allow for a min-heap
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for PathNodePos {
    fn partial_cmp(&self, other: &PathNodePos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
