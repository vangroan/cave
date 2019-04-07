use piston::input::RenderArgs;

#[derive(Default)]
pub struct DeltaTime(pub f64);

/// New Type for RenderArgs
pub struct OnRender(RenderArgs);

impl OnRender {
    pub fn new(args: RenderArgs) -> Self {
        OnRender(args)
    }

    #[inline(always)]
    pub fn args(&self) -> &RenderArgs {
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
