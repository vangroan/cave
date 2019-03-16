use nalgebra::Vector3;
use specs::{Read, System, Write};

use crate::grid::{grid_index, grid_index_u, Grid, GridPosition};
use crate::option::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};

fn manhatten(a: &GridPosition, b: &GridPosition) -> u32 {
    ((a.x() - b.x()).abs() + (a.y() - b.y()).abs() + (a.z() - b.z()).abs()) as u32
}

pub struct Pathfinder;

impl Pathfinder {
    pub fn new() -> Pathfinder {
        Pathfinder
    }

    pub fn find_path(&self, grid: &Grid, start: &GridPosition, end: &GridPosition) -> Option<Path> {
        // Note the BinaryHeap is a max-heap
        let mut nodes = PathSpace::from_grid(&grid);
        let mut open: BinaryHeap<PathNodePos> = BinaryHeap::new();
        let mut close: HashSet<GridPosition> = HashSet::new();

        // Seed lists with initial position
        let start_h = manhatten(start, end);
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
            // We keep track of how fra we've traveled from the start
            let node_g: u32;
            {
                let (node, _parent) = nodes
                    .get(&node_pos)
                    .expect("Popped node from priority queue that's not in the known space");
                node_g = node.g;
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

                // Check if we've reached our destination
                if &node_pos == end {
                    // Trace the path back to start
                    return Some(Pathfinder::trace_path(&end, &mut nodes));
                }

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

        None
    }

    fn trace_path(end: &GridPosition, space: &mut PathSpace) -> Path {
        let mut next_pos = end.clone();
        let mut result = Vec::<PathNode>::new();

        'walk: while let Some((node, maybe_parent)) = space.take(&next_pos) {
            result.push(node);
            match maybe_parent {
                Some(parent_pos) => next_pos = parent_pos,
                None => break 'walk,
            }
        }

        // Tracing is backwards, from end to start
        result.reverse();
        Path(result)
    }
}

impl Default for Pathfinder {
    fn default() -> Pathfinder {
        Pathfinder
    }
}

pub struct Path(Vec<PathNode>);

/// Container to hold nodes that have been searched
///
/// Nodes are stored with an optional parent position, which is used
/// to track the path of nodes.
struct PathSpace {
    data: Vec<Option<(PathNode, Option<GridPosition>)>>,
    size: Vector3<u32>,
}

impl PathSpace {
    fn with_size(x: u32, y: u32, z: u32) -> PathSpace {
        let data: Vec<Option<(PathNode, Option<GridPosition>)>> =
            (0..(x * y * z)).map(|_| None).collect();

        PathSpace {
            data,
            size: Vector3::new(x, y, z),
        }
    }

    fn from_grid(grid: &Grid) -> PathSpace {
        let size = grid.size();
        PathSpace::with_size(size.0, size.1, size.2)
    }

    fn get(&self, pos: &GridPosition) -> Option<&(PathNode, Option<GridPosition>)> {
        let index = grid_index(&self.size, &pos);
        match self.data.get(index) {
            Some(in_bounds_node) => in_bounds_node.as_ref(),
            None => None,
        }
    }

    fn take(&mut self, pos: &GridPosition) -> Option<(PathNode, Option<GridPosition>)> {
        let index = grid_index(&self.size, &pos);
        let mut result: Option<(PathNode, Option<GridPosition>)> = None;
        ::std::mem::swap(&mut result, &mut self.data[index]);
        result
    }

    fn get_parent(&self, pos: &GridPosition) -> Option<&PathNode> {
        self.get(pos)
            .and_then(|(_node, maybe_parent)| maybe_parent.as_ref())
            .and_then(|parent_pos| self.get(parent_pos))
            .and_then(|(parent_node, _)| Some(parent_node))
    }

    fn set(&mut self, node: PathNode, parent: Option<GridPosition>) {
        let index = grid_index(&self.size, &node.pos);
        self.data[index] = Some((node, parent));
    }

    fn clear(&mut self) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                for z in 0..self.size.z {
                    self.data[grid_index_u(&self.size, &(x, y, z))] = None;
                }
            }
        }
    }
}

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
struct PathNodePos(GridPosition, u32);

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

#[derive(Default)]
pub struct PathRequests(pub VecDeque<(GridPosition, GridPosition)>);

impl PathRequests {
    pub fn new() -> Self {
        PathRequests(VecDeque::new())
    }
}

#[derive(Default)]
pub struct PathResults(pub VecDeque<(GridPosition, GridPosition)>);

impl PathResults {
    pub fn new() -> Self {
        PathResults(VecDeque::new())
    }
}

pub struct PathfindingSystem;

impl<'a> System<'a> for PathfindingSystem {
    type SystemData = (
        Read<'a, Pathfinder>,
        Write<'a, PathRequests>,
        Write<'a, PathResults>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use rayon::prelude::*;
        use specs::Join;
        use specs::ParJoin;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_pathfinding() {
        let grid = Grid::with_size(128, 128, 128);
        let pathfinder = Pathfinder::new();

        {
            let path = pathfinder
                .find_path(
                    &grid,
                    &GridPosition::new(0, 0, 0),
                    &GridPosition::new(10, 10, 0),
                )
                .expect("Failed to find path");
            assert_eq!(GridPosition::new(0, 0, 0), path.0[0].pos);
            assert_eq!(GridPosition::new(2, 2, 0), path.0[2].pos);
            assert_eq!(GridPosition::new(5, 5, 0), path.0[5].pos);
            assert_eq!(GridPosition::new(10, 10, 0), path.0[10].pos);
            assert_eq!(11, path.0.len());
        }
    }
}
