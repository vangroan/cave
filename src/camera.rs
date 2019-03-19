
use specs::prelude::*;

/// Isometric camera
/// 
/// It's 3D position is it's lookat position
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct IsometricCamera {
    current: bool,
}

impl IsometricCamera {
    pub fn new(current: bool) -> Self {
        IsometricCamera { current }
    }

    #[inline(always)]
    pub fn is_current(&self) -> bool {
        self.current
    }
}
