pub struct Settings {
    pub title: &'static str,
    pub window_width: u32,
    pub window_height: u32,

    /// Updates per second
    pub ups: u64,

    /// Renders per second
    pub fps: u64,
}

impl Settings {
    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn fps(mut self, fps: u64) -> Self {
        self.fps = fps;
        self
    }

    pub fn ups(mut self, ups: u64) -> Self {
        self.ups = ups;
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            title: "app",
            window_width: 640,
            window_height: 480,
            ups: 60,
            fps: 60,
        }
    }
}
