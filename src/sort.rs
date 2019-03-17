use opengl_graphics::Texture;
use specs::prelude::*;

use crate::pigeon::PigeonholeSort;
use crate::position::Position;
use crate::sprite::Sprite;

/// Sorts objects according to the isometric projection
pub struct IsometricSorter {
    sort: PigeonholeSort<DepthItem>,
}

impl IsometricSorter {
    pub fn with_size(x: u32, y: u32, z: u32) -> Self {
        IsometricSorter {
            sort: PigeonholeSort::new(0, (x + y + z) as i32),
        }
    }
}

impl<'a> System<'a> for IsometricSorter {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Sprite<Texture>>,
        Write<'a, DepthBuffer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (entities, positions, mut sprites, mut buffer) = data;
        let IsometricSorter { sort } = self;

        let mut unsorted: Vec<DepthItem> = vec![];

        for (e, position, sprite) in (&entities, &positions, &mut sprites).join() {
            // Calculate depth
            // TODO: Once we've defined anchor points and tile subdivions, this should be more intricate
            let depth = (position.x().floor() + position.y().floor() + position.z().floor()) as i32;
            sprite.set_depth(depth);
            unsorted.push(DepthItem {
                depth,
                entity_id: e.id(),
            });
        }

        // Ensure output buffer is clean and large enough
        buffer.0.clear();
        for _ in 0..unsorted.len() {
            buffer.0.push(Default::default());
        }

        sort.sort_into(&mut unsorted, &mut buffer.0, |item| item.depth);
    }
}

#[derive(Default)]
pub struct DepthItem {
    entity_id: u32,
    depth: i32,
}

impl DepthItem {
    pub fn entity_id(&self) -> u32 {
        self.entity_id
    }
}

#[derive(Default)]
pub struct DepthBuffer(Vec<DepthItem>);

impl DepthBuffer {
    pub fn new() -> Self {
        DepthBuffer(vec![])
    }

    pub fn contents(&self) -> &[DepthItem] {
        &self.0
    }
}
