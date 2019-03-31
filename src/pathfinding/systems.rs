use super::astar::AStar;
use super::components::*;
use super::cost::Cost;
use super::pathfinder::*;
use super::tilemap::*;
use crate::grid::Grid;
use crate::grid::GridPosition;
use crate::pathfinding::path_result::PathResult;
use crate::tilemap::Tilemap;
use specs::prelude::*;

pub struct PathfindingSystem;

impl PathfindingSystem {
    pub fn new() -> Self {
        PathfindingSystem
    }
}

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        // TODO: Polymorphic pathfinding
        Read<'a, AStar>,
        Read<'a, Grid>,
        Read<'a, Tilemap>,
        WriteStorage<'a, Pather>,
    );

    fn run(&mut self, (pathfinder, grid, tilemap, mut pathers): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;

        let cost_strat = TilemapCost::new(&tilemap);
        let loco_strat = TilemapLocomotion::new(&tilemap);

        // TODO: Parallel join is not reaching rayon threshold, so runs synchronously regardless
        (&mut pathers)
            .par_join()
            .filter(|pather| pather.needs_path())
            .for_each(|pather| {
                let maybe_request = pather.take_request();
                if let PathRequest::Request(start, end) = maybe_request {
                    println!("pathfinding thread: {:?}", ::std::thread::current().id());

                    let path_result =
                        pathfinder.find_path(&grid, &start, &end, &cost_strat, &loco_strat);
                        
                    if path_result.is_success() {
                        pather.set_request(PathRequest::Ready(path_result));
                    } else {
                        pather.set_request(PathRequest::Failed);
                    }
                }
            });
    }
}
