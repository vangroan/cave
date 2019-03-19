
use specs::prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Actor;

impl Actor {
    pub fn new() -> Self {
        Actor
    }
}
