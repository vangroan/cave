use opengl_graphics::Texture;
use specs::prelude::*;

use crate::position::Position;
use crate::sprite::Sprite;

/// Sorts objects according to the isometric projection
pub struct IsometricSorter;

impl IsometricSorter {
    pub fn new() -> Self {
        IsometricSorter
    }
}

impl<'a> System<'a> for IsometricSorter {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Sprite<Texture>>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, sprites) = data;
    }
}
