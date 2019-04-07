use opengl_graphics::{GlGraphics, OpenGL};

/// Wrapper resource for Graphics object.
///
/// Allows passing graphics to systems.
pub struct Graphix(GlGraphics);

impl Graphix {
    pub fn new(gl: GlGraphics) -> Self {
        Graphix(gl)
    }

    #[inline(always)]
    pub fn gl_mut(&mut self) -> &mut GlGraphics {
        &mut self.0
    }
}

impl Default for Graphix {
    fn default() -> Self {
        Graphix(GlGraphics::new(OpenGL::V3_2))
    }
}