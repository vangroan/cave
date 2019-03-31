use na::Vector3;

use crate::grid::{grid_index, grid_index_u, Grid, GridPosition};

use super::path_node::PathNode;

/// Container to hold nodes that have been searched
///
/// Nodes are stored with an optional parent position, which is used
/// to track the path of nodes.
///
/// This is required because we can't easily build the path space with
/// pointers between nodes
pub struct PathSpace {
    data: Vec<Option<(PathNode, Option<GridPosition>)>>,
    size: Vector3<u32>,
}

impl PathSpace {
    pub fn with_size(x: u32, y: u32, z: u32) -> PathSpace {
        let data: Vec<Option<(PathNode, Option<GridPosition>)>> =
            (0..(x * y * z)).map(|_| None).collect();

        PathSpace {
            data,
            size: Vector3::new(x, y, z),
        }
    }

    pub fn from_grid(grid: &Grid) -> PathSpace {
        let size = grid.size();
        PathSpace::with_size(size.0, size.1, size.2)
    }

    pub fn get(&self, pos: &GridPosition) -> Option<&(PathNode, Option<GridPosition>)> {
        let index = grid_index(&self.size, &pos);
        match self.data.get(index) {
            Some(in_bounds_node) => in_bounds_node.as_ref(),
            None => None,
        }
    }

    pub fn take(&mut self, pos: &GridPosition) -> Option<(PathNode, Option<GridPosition>)> {
        let index = grid_index(&self.size, &pos);
        let mut result: Option<(PathNode, Option<GridPosition>)> = None;
        ::std::mem::swap(&mut result, &mut self.data[index]);
        result
    }

    pub fn get_parent(&self, pos: &GridPosition) -> Option<&PathNode> {
        self.get(pos)
            .and_then(|(_node, maybe_parent)| maybe_parent.as_ref())
            .and_then(|parent_pos| self.get(parent_pos))
            .and_then(|(parent_node, _)| Some(parent_node))
    }

    pub fn set(&mut self, node: PathNode, parent: Option<GridPosition>) {
        let index = grid_index(&self.size, &node.pos);
        self.data[index] = Some((node, parent));
    }

    pub fn clear(&mut self) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                for z in 0..self.size.z {
                    self.data[grid_index_u(&self.size, &(x, y, z))] = None;
                }
            }
        }
    }

    pub fn carve_path(&mut self, end: &GridPosition) -> Vec<PathNode> {
        let mut next_pos = end.clone();
        let mut result = Vec::<PathNode>::new();

        'walk: while let Some((node, maybe_parent)) = self.take(&next_pos) {
            result.push(node);
            match maybe_parent {
                Some(parent_pos) => next_pos = parent_pos,
                None => break 'walk,
            }
        }

        // Tracing is backwards, from end to start
        result.reverse();
        result
    }
}
