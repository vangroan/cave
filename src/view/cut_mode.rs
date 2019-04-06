use crate::position::Position;

pub struct ViewCutMode {
    mode: CutMode,
    in_vector: na::Vector3<f64>,
}

impl ViewCutMode {
    pub fn new(mode: CutMode) -> Self {
        let in_vector = mode.vector();
        ViewCutMode { mode, in_vector }
    }

    /// Given a 3D position, determine whether it's inside the
    /// cut view.
    #[inline(always)]
    pub fn is_outside(&self, cut_point: &Position, pos: &Position) -> bool {
        let camera_subtracted = pos.to_vector() - cut_point.to_vector();
        self.in_vector.dot(&camera_subtracted) < 0.0
    }

    pub fn mode(&self) -> CutMode {
        self.mode.clone()
    }

    pub fn set_mode(&mut self, mode: CutMode) {
        self.in_vector = mode.vector();
        self.mode = mode;
    }
}

impl Default for ViewCutMode {
    fn default() -> Self {
        ViewCutMode::new(CutMode::Top)
    }
}

#[derive(Clone)]
pub enum CutMode {
    Top = 0,
    Left,
    Right,
}

impl CutMode {
    fn vector(&self) -> na::Vector3<f64> {
        use CutMode::*;

        match self {
            Top => na::Vector3::new(0., 0., -1.),
            Left => na::Vector3::new(0., -1., 0.),
            Right => na::Vector3::new(-1., 0., 0.),
        }
    }
}
