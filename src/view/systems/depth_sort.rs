use super::super::depth_graph::DepthGraph;
use specs::{System, Write};

pub struct DepthSortSystem;

impl<'a> System<'a> for DepthSortSystem {
    type SystemData = (Write<'a, DepthGraph>,);

    fn run(&mut self, data: Self::SystemData) {}
}
