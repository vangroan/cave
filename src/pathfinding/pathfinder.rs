use crate::grid::{Grid, GridPosition};

use super::cost::Cost;
use super::path_result::PathResult;

pub trait Pathfinder {
    fn find_path<F>(&self, grid: &Grid, start: &GridPosition, end: &GridPosition, cost_func: F) -> PathResult
    where
        F: Fn(&GridPosition, &GridPosition) -> Cost;
}
