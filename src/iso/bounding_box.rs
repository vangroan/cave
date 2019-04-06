use specs::prelude::*;

/// A convex box around an object in 3-dimensional space.
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct BoundingBox {
    pos: na::Vector3<f64>,
    size: na::Vector3<f64>,
}

impl BoundingBox {
    pub fn new(pos: na::Vector3<f64>, size: na::Vector3<f64>) -> Self {
        BoundingBox { pos, size }
    }

    /// Indicates if two bounding boxes overlap in 3D space
    pub fn intersect(&self, other: &BoundingBox) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    pub fn position(&self) -> &na::Vector3<f64> {
        &self.pos
    }

    #[inline(always)]
    pub fn size(&self) -> &na::Vector3<f64> {
        &self.size
    }

    #[inline(always)]
    pub fn p0(&self) -> na::Vector3<f64> {
        self.pos.clone()
    }

    #[inline(always)]
    pub fn p1(&self) -> na::Vector3<f64> {
        na::Vector3::new(self.pos.x + self.size.x, self.pos.y, self.pos.z)
    }

    #[inline(always)]
    pub fn p2(&self) -> na::Vector3<f64> {
        na::Vector3::new(
            self.pos.x + self.size.x,
            self.pos.y + self.size.y,
            self.pos.z,
        )
    }

    #[inline(always)]
    pub fn p3(&self) -> na::Vector3<f64> {
        na::Vector3::new(self.pos.x, self.pos.y + self.size.y, self.pos.z)
    }

    #[inline(always)]
    pub fn p4(&self) -> na::Vector3<f64> {
        na::Vector3::new(self.pos.x, self.pos.y, self.pos.z + self.size.z)
    }

    #[inline(always)]
    pub fn p5(&self) -> na::Vector3<f64> {
        na::Vector3::new(
            self.pos.x + self.size.x,
            self.pos.y,
            self.pos.z + self.size.z,
        )
    }

    #[inline(always)]
    pub fn p6(&self) -> na::Vector3<f64> {
        na::Vector3::new(
            self.pos.x + self.size.x,
            self.pos.y + self.size.y,
            self.pos.z + self.size.z,
        )
    }

    #[inline(always)]
    pub fn p7(&self) -> na::Vector3<f64> {
        na::Vector3::new(
            self.pos.x,
            self.pos.y + self.size.y,
            self.pos.z + self.size.z,
        )
    }
}
