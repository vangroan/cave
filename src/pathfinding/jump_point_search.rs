
use super::pathfinder::Pathfinder;

pub struct JumpPointSearch;

impl Pathfinder for JumpPointSearch {
    fn find_path<F>(&self, grid: &Grid, start: &GridPosition, end: &GridPosition) -> PathResult
    where
        F: Fn(&GridPosition, &GridPosition) -> Cost
        {
            unimplemented!()
        }
}
