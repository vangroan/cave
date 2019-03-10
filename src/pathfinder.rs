
use specs::{System, Write, Read};

use super::grid::{Grid, GridPosition};

use std::collections::VecDeque;

pub struct Pathfinder;

impl Pathfinder {
    pub fn new() -> Pathfinder {
        Pathfinder
    }

    pub fn find_path(&self, grid: &Grid, start: &GridPosition, end: &GridPosition) {

    }
}

impl Default for Pathfinder {
    fn default() -> Pathfinder {
        Pathfinder
    }
}

pub struct Path(Vec<PathNode>);

pub struct PathNode {
    pub pos: GridPosition,
    pub parent: GridPosition,
    /// distance from start
    pub g: u32,
    /// heuristic
    pub h: u32,
    /// total cost
    pub cost: u32,
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
