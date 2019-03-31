//! A* Pathfinding

use super::cost::Cost;
use super::distance::*;
use super::path_node::*;
use super::path_result::PathResult;
use super::path_space::PathSpace;
use super::pathfinder::Pathfinder;
use crate::grid::{Grid, GridPosition};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time;

#[derive(Default)]
pub struct AStar;

impl AStar {
    pub fn new() -> AStar {
        AStar
    }
}

impl Pathfinder for AStar {
    fn find_path<F>(
        &self,
        grid: &Grid,
        start: &GridPosition,
        end: &GridPosition,
        cost_func: F,
    ) -> PathResult
    where
        F: Fn(&GridPosition, &GridPosition) -> Cost,
    {
        let mut iter_count = 0;
        let start_time = time::Instant::now();

        // Note the BinaryHeap is a max-heap
        let mut nodes = PathSpace::from_grid(&grid);
        let mut open: BinaryHeap<PathNodePos> = BinaryHeap::new();
        let mut close: HashSet<GridPosition> = HashSet::new();

        // Seed lists with initial position
        let start_h = euler(start, end);
        let start_node = PathNode {
            pos: start.clone(),
            g: 0,
            h: start_h,
            cost: start_h,
        };
        nodes.set(start_node, None);
        open.push(PathNodePos(start.clone(), start_h));
        close.insert(start.clone());

        while let Some(PathNodePos(node_pos, _)) = open.pop() {
            iter_count += 1;

            // We keep track of how far we've traveled from the start
            let node_g: u32;
            {
                let (node, _parent) = nodes
                    .get(&node_pos)
                    .expect("Popped node from priority queue that's not in the known space");
                node_g = node.g;
            }

            // Check if we've reached our destination
            if &node_pos == end {
                // Trace the path back to start
                return PathResult::with_stats(
                    iter_count,
                    time::Instant::now().duration_since(start_time),
                    nodes.carve_path(&end),
                );
            }

            let neighbours = grid.neighbours(&node_pos);
            let in_bound_neighbours = neighbours
                .into_iter()
                .filter_map(|maybe_neigh| maybe_neigh.as_ref());

            for neigh_pos in in_bound_neighbours {
                // Disregard nodes we have seen before
                if close.contains(&neigh_pos) {
                    continue;
                }

                // TODO: Rethink how to pass in Cost func so we don't have all these ownership issues with tilemap
                let cost = cost_func(&node_pos, &neigh_pos);
                if cost == Cost::Blocked {
                    continue;
                }

                // TODO: Corner cutting detection

                let g = &node_g + 1;
                let h = manhatten(&node_pos, neigh_pos);

                // TODO: Check if node is pathable
                let parent_node = Some(node_pos.clone());
                let new_node = PathNode {
                    pos: neigh_pos.clone(),
                    g: g,
                    h: h,
                    cost: g + h,
                };

                open.push(PathNodePos(new_node.pos.clone(), new_node.cost));
                close.insert(neigh_pos.clone());
                nodes.set(new_node, parent_node);
            }
        }

        PathResult::with_fail_stats(iter_count, time::Instant::now().duration_since(start_time))
    }
}
