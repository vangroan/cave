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
        locomotion: &Locomotion,
        start: &GridPosition,
        end: &GridPosition,
        cost_strat: &C,
        loco_strat: &L,
    ) -> PathResult
    where
        C: CostStrategy,
        L: LocomotionStrategy,
    {
        unimplemented!()
    }
}
