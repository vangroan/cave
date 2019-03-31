use crate::grid::{Grid, GridPosition};

use super::cost::*;
use super::locomotion::*;
use super::path_result::PathResult;
use super::pathfinder::Pathfinder;

pub struct JumpPointSearch;

impl Pathfinder for JumpPointSearch {
    fn find_path<C, L>(
        &self,
        grid: &Grid,
        start: &GridPosition,
        end: &GridPosition,
        cost: &C,
        locomotion: &L,
    ) -> PathResult
    where
        C: CostStrategy,
        L: LocomotionStrategy,
    {
        unimplemented!()
    }
}
