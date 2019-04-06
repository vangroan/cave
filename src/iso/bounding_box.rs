use specs::prelude::*;

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
}
