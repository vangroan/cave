
use specs::{System, Write, Read};

use super::grid::{Grid, GridPosition};

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

    pub fn find_path(&self, grid: &Grid, start: &GridPosition, end: &GridPosition) {
        // Note the BinaryHeap is a max-heap
        let mut open : BinaryHeap<PathNode> = BinaryHeap::new();
        let mut close : HashSet<GridPosition> = HashSet::new();

        // Seed lists with initial position
        let start_h = manhatten(start, end);
        open.push(PathNode {
            pos :start.clone(),
            parent: None,
            g: 0,
            h: start_h,
            cost: start_h,
        });
        close.insert(start.clone());


        while let Some(node) = open.pop() {

            for neigh in grid.neighbours(&node.pos).into_iter() {

            }
        }
    }
}

impl Default for Pathfinder {
    fn default() -> Pathfinder {
        Pathfinder
    }
}

pub struct Path(Vec<PathNode>);

#[derive(Eq, PartialEq)]
pub struct PathNode {
    pub pos: GridPosition,
    pub parent: Option<GridPosition>,
    /// distance from start
    pub g: u32,
    /// heuristic
    pub h: u32,
    /// total cost
    pub cost: u32,
}

impl Ord for PathNode {
    fn cmp(&self, other: &PathNode) -> Ordering {
        // Note that this is backwards to allow for a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &PathNode) -> Option<Ordering> {
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
