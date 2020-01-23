use daggy::Dag;

/// Spatial index for the 2-dimentional screen space.
pub struct QuadTree<T> {
    graph: Dag<TreeNode<T>, TreeEdge>,
    max_levels: u32,
    max_items: u32,
}

impl<T> QuadTree<T> {
    pub fn new(max_levels: u32, max_items: u32) -> Self {
        QuadTree {
            graph: Dag::new(),
            max_levels,
            max_items,
        }
    }

    pub fn add(&mut self, item: T) {
        unimplemented!()
    }

    pub fn remove(&mut self, item: T) {
        unimplemented!()
    }

    fn split(&mut self) {
        unimplemented!()
    }

    fn merge(&mut self) {
        unimplemented!()
    }
}

impl<T> Default for QuadTree<T> {
    fn default() -> Self {
        QuadTree::new(5, 10)
    }
}

enum TreeNode<T> {
    Branch { level: u32 },
    Leaf { items: Vec<T> },
}

struct TreeEdge;
