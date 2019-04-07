
use specs::{System, Write};
use super::super::depth_graph::DepthGraph;

pub struct DepthSortSystem;

impl<'a> System<'a> for DepthSortSystem {
    type SystemData = (
        Write<'a, DepthGraph>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}
