use std::time::Duration;

use super::path_node::PathNode;

pub struct PathResult {
    /// Metric for number of iterations
    iter_count: u32,

    /// Metric for search time taken
    duration: Duration,

    /// Actual path resulting from the search
    ///
    /// `None` if the search failed
    path: Option<Vec<PathNode>>,
}

impl PathResult {
    pub fn path(&self) -> Option<&Vec<PathNode>> {
        self.path.as_ref()
    }
}
