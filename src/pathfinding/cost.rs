
use crate::grid::GridPosition;

#[derive(Eq, PartialEq)]
pub enum Cost {
    Passable(u32),
    Blocked,
}

/// A strategy passed into the pathfinding function to apply pathing rules
/// based on the cost of moving accross the terrain.
pub trait CostStrategy {
    /// Indicates whether the pather can travel from the source position to
    /// the target position, and the expected cost of the movement.
    fn is_passable(&self, source: &GridPosition, target: &GridPosition) -> Cost;
}
