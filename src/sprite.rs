use std::sync::Arc;

use graphics::types::{Color, Matrix2d};
use graphics::{Graphics, ImageSize, Transformed};
use na::Vector2;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::*;
use specs::prelude::*;

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

pub struct SpriteRenderer {
    gl: GlGraphics,
}

impl SpriteRenderer {
    pub fn from_graphics(gl: GlGraphics) -> Self {
        SpriteRenderer { gl }
    }
}

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = (Read<'a, OnRender>, ReadStorage<'a, Sprite<Texture>>);

    fn run(&mut self, data: Self::SystemData) {
        use graphics::*;
        use specs::Join;

        let SpriteRenderer { gl, .. } = self;
        let (on_render, sprites) = data;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        gl.draw(on_render.args().viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // TODO: Camera
            let transform = c.transform;

            for sprite in sprites.join() {
                sprite.draw(transform, gl);
            }
        });
    }
}

/// New Type for RenderArgs
pub struct OnRender(RenderArgs);

impl OnRender {
    pub fn new(args: RenderArgs) -> Self {
        OnRender(args)
    }

    fn args(&self) -> &RenderArgs {
        &self.0
    }
}

impl Default for OnRender {
    fn default() -> Self {
        OnRender(RenderArgs {
            /// Extrapolated time in seconds, used to do smooth animation.
            ext_dt: 0.,
            /// The width of rendered area in points.
            width: 0.,
            /// The height of rendered area in points.
            height: 0.,
            /// The width of rendered area in pixels.
            draw_width: 0,
            /// The height of rendered area in pixels.
            draw_height: 0,
        })
    }
}
