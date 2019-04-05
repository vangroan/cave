use opengl_graphics::Texture;
use specs::prelude::*;

use crate::isometric::Isometric;
use crate::pigeon::PigeonholeSort;
use crate::position::Position;
use crate::settings::flatten_pos;
use crate::sprite::Sprite;
use crate::view::components::IsometricCamera;

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
        ReadStorage<'a, IsometricCamera>,
        WriteStorage<'a, Sprite<Texture>>,
        Write<'a, DepthBuffer>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (entities, positions, cameras, mut sprites, mut buffer) = data;
        let IsometricSorter { sort } = self;
        let maybe_camera = (&cameras, &positions)
            .join()
            .find(|(camera, _)| camera.is_current());

        if let Some((_camera, camera_pos)) = maybe_camera {
            const VIEWPORT_WIDTH: f64 = 640.;
            const VIEWPORT_HEIGHT: f64 = 480.;
            let camera_pos_iso = Isometric::cart_to_iso(&camera_pos.to_vector());
            // println!("Camera Position: ({}, {}, {})", camera_pos_iso.x, camera_pos_iso.y, camera_pos_iso.z);
            let camera_pos_2d = flatten_pos(&camera_pos_iso);
            println!(
                "Camera Position: pos({}, {}, {}) iso({}, {}, {}) screen({}, {})",
                camera_pos.x(),
                camera_pos.y(),
                camera_pos.z(),
                camera_pos_iso.x,
                camera_pos_iso.y,
                camera_pos_iso.z,
                camera_pos_2d.x,
                camera_pos_2d.y
            );
            let tile_rect_2d = (
                camera_pos_2d.x - (VIEWPORT_WIDTH / 2.),
                camera_pos_2d.y - (VIEWPORT_HEIGHT / 2.),
                VIEWPORT_WIDTH,
                VIEWPORT_HEIGHT,
            );

            let mut unsorted: Vec<DepthItem> = vec![];

            for (e, position, sprite) in (&entities, &positions, &mut sprites).join() {
                // Determine if the sprite is visible in 2D screen space
                let tile_pos_iso = Isometric::cart_to_iso(&position.to_vector());
                let tile_pos_2d = flatten_pos(&tile_pos_iso);

                if !(tile_pos_2d.x >= tile_rect_2d.0
                    && tile_pos_2d.x <= tile_rect_2d.0 + tile_rect_2d.2
                    && tile_pos_2d.y >= tile_rect_2d.1
                    && tile_pos_2d.y <= tile_rect_2d.1 + tile_rect_2d.3)
                {
                    // Not in viewport
                    continue;
                }

                // Calculate depth
                // TODO: Once we've defined anchor points and tile subdivions, this should be more intricate
                let depth =
                    (position.x().floor() + position.y().floor() + position.z().floor()) as i32;
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
