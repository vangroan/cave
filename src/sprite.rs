use std::sync::Arc;

use graphics::types::{Color, Matrix2d};
use graphics::{Graphics, ImageSize, Transformed};
use na::Vector2;
use opengl_graphics::Texture;
use piston::input::*;
use specs::prelude::*;

use crate::common::components::OnRender;
use crate::depthsort::DepthBuffer;
use crate::isometric::Isometric;
use crate::position::Position;
use crate::render::Graphix;
use crate::settings::*;
use crate::view::components::IsometricCamera;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Sprite<I: ImageSize + Send + Sync + 'static> {
    translate: Vector2<f64>,
    scale: Vector2<f64>,
    rotation: f64,
    anchor: Vector2<f64>,
    depth: i32,
    src_rect: Option<[f64; 4]>,
    color: Color,
    tex: Arc<I>,
}

impl<I> Sprite<I>
where
    I: ImageSize + Send + Sync + 'static,
{
    pub fn from_texture(tex: Arc<I>) -> Self {
        Sprite {
            translate: Vector2::new(0., 0.),
            scale: Vector2::new(1., 1.),
            rotation: 0.,
            anchor: Vector2::new(0., 0.),
            depth: 0,
            src_rect: None,
            color: [1., 1., 1., 1.],
            tex,
        }
    }

    #[inline(always)]
    pub fn position(&self) -> &Vector2<f64> {
        &self.translate
    }

    #[inline(always)]
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.translate = Vector2::new(x, y)
    }

    #[inline(always)]
    pub fn anchor(&self) -> &Vector2<f64> {
        &self.anchor
    }

    #[inline(always)]
    pub fn set_anchor(&mut self, x: f64, y: f64) {
        self.anchor = Vector2::new(x, y)
    }

    #[inline(always)]
    pub fn depth(&mut self) -> i32 {
        self.depth
    }

    #[inline(always)]
    pub fn set_depth(&mut self, depth: i32) {
        self.depth = depth
    }

    #[inline(always)]
    pub fn color(&self) -> &Color {
        &self.color
    }

    #[inline(always)]
    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn draw<G>(&self, transform: Matrix2d, g: &mut G)
    where
        G: Graphics<Texture = I>,
    {
        let (tex_w, tex_h) = self.tex.get_size();
        let tex_w = tex_w as f64;
        let tex_h = tex_h as f64;
        let source_rectangle = self.src_rect.unwrap_or({
            let (w, h) = (tex_w, tex_h);
            [0.0, 0.0, w as f64, h as f64]
        });

        let anchor = [
            self.anchor[0] * source_rectangle[2],
            self.anchor[1] * source_rectangle[3],
        ];

        let t = transform
            .trans(self.translate.x, self.translate.y)
            .rot_deg(self.rotation)
            .scale(self.scale.x, self.scale.y);

        let ref draw_state: graphics::DrawState = Default::default();

        graphics::Image::new()
            .color(self.color)
            .rect([
                -anchor[0],
                -anchor[1],
                source_rectangle[2],
                source_rectangle[3],
            ])
            .maybe_src_rect(self.src_rect)
            .draw(&*self.tex, draw_state, t, g);
    }
}

pub struct SpriteRenderer;

impl SpriteRenderer {
    pub fn new() -> Self {
        SpriteRenderer
    }
}

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = (
        Entities<'a>,
        Read<'a, OnRender>,
        Write<'a, Graphix>,
        ReadStorage<'a, IsometricCamera>,
        ReadStorage<'a, Sprite<Texture>>,
        ReadStorage<'a, Position>,
        Read<'a, DepthBuffer>,
    );

    fn run(
        &mut self,
        (entities, on_render, mut graphics, cameras, sprites, positions, buffer): Self::SystemData,
    ) {
        use graphics::*;
        use specs::Join;

        let gl = graphics.gl_mut();

        let camera_pos_iso = (&cameras, &positions)
            .join()
            .find(|(camera, _position)| camera.is_current())
            .map(|(_camera, position)| Isometric::cart_to_iso(position.to_vector()))
            .unwrap_or(na::Vector3::new(0., 0., 0.));
        // let camera_pos_2d = na::Vector2::<f64>::new(camera_pos_iso.x, camera_pos_iso.y + camera_pos_iso.z);
        let camera_pos_2d = flatten_pos(&camera_pos_iso);

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);

        gl.draw(on_render.args().viewport(), |c, gl| {
            // Center of screen
            let (offset_x, offset_y) =
                (on_render.args().width / 2.0, on_render.args().height / 2.0);

            // Clear the screen.
            clear(BLACK, gl);

            // Positions are inversed, because world is transformed in the reverse direction
            // of what the camera is doing
            let transform = c
                .transform
                .trans(-(camera_pos_2d.x) + offset_x, -(camera_pos_2d.y) + offset_y);

            for item in buffer.contents() {
                let e = entities.entity(item.entity_id());
                if let Some(sprite) = sprites.get(e) {
                    if let Some(pos) = positions.get(e) {
                        let iso_pos = Isometric::cart_to_iso(&pos.to_vector());
                        let screen_pos = flatten_pos(&iso_pos);
                        sprite.draw(transform.trans(screen_pos.x, screen_pos.y), gl);
                    }
                }
            }

            // Center of world
            rectangle(RED, square, transform, gl);

            // Center of camera view
            rectangle(
                GREEN,
                square,
                transform.trans(camera_pos_2d.x, camera_pos_2d.y),
                gl,
            );
        });
    }
}
