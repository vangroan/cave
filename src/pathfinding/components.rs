use specs::prelude::*;

use crate::grid::GridPosition;

use super::path_node::PathNode;
use super::path_result::PathResult;

/// Marks an Entity as being able to search paths
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Pather {
    cursor: usize,
    request: PathRequest,
}

impl Pather {
    pub fn new() -> Self {
        Pather {
            cursor: 0,
            request: PathRequest::Nothing,
        }
    }

    pub fn with_request(start: GridPosition, end: GridPosition) -> Pather {
        Pather {
            cursor: 0,
            request: PathRequest::Request(start, end),
        }
    }

    pub fn needs_path(&self) -> bool {
        match self.request {
            PathRequest::Request(_, _) => true,
            _ => false,
        }
    }

    pub fn has_path(&self) -> bool {
        match self.request {
            PathRequest::Ready(_) => true,
            _ => false,
        }
    }

    pub fn take_request(&mut self) -> PathRequest {
        let mut request = PathRequest::Nothing;
        ::std::mem::swap(&mut request, &mut self.request);
        request
    }

    #[inline(always)]
    pub fn request(&self) -> &PathRequest {
        &self.request
    }

    #[inline(always)]
    pub fn set_request(&mut self, req: PathRequest) {
        self.request = req;
    }

    pub fn next(&mut self) -> Option<&PathNode> {
        if let PathRequest::Ready(ref path_result) = self.request {
            if let Some(node) = path_result.path().and_then(|p| p.get(self.cursor)) {
                self.cursor += 1;
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn current(&self) -> Option<&PathNode> {
        if let PathRequest::Ready(ref path_result) = self.request {
            path_result.path().and_then(|p| p.get(self.cursor))
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.request = PathRequest::Nothing;
    }
}

pub enum PathRequest {
    Request(GridPosition, GridPosition),
    Ready(PathResult),
    Failed,
    Nothing,
}
