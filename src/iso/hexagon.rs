use super::bounding_box::BoundingBox;
use specs::prelude::*;

/// Represents an isometric bounding box, flattened
/// to 2D space.
///
/// It is used for determining whether objects overlap
/// in 2D space. It stores the minium and maximum `x` and `y`
/// coordinates from 3D space, with the `z` added to both.
///
/// It also stores the minimum and maximum *horizontol* 2D
/// coordinate.
///
/// The hexagon does not keep position information, and thus
/// its position within the world must be passed in with each
/// intersection test;
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Hexagon {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
    min_h: f64,
    max_h: f64,
}

impl Hexagon {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64, min_h: f64, max_h: f64) -> Self {
        Hexagon {
            min_x,
            min_y,
            max_x,
            max_y,
            min_h,
            max_h,
        }
    }

    /// Creates a hexagon using a 3D bounding box
    ///
    /// Because the `z` coordinate moves a tile upwards in 2D space, we
    /// can combine it with both `x` and `y`, and still get correct
    /// intersection tests.
    ///
    /// This means we won't be able to get the `z` component back out, or
    /// onvert the hexagon back to a bounding box.
    pub fn from_bounding_box(bounding_box: &BoundingBox) -> Self {
        use std::f64::consts::PI;

        let pos = bounding_box.position();
        let size = bounding_box.size();

        let (min_x, min_y) = (pos.x + pos.z, pos.y + pos.z);
        let (max_x, max_y) = (
            pos.x + size.x + pos.z + size.z,
            pos.y + size.y + pos.z + size.z,
        );
        let (min_h, max_h) = (
            (min_x - min_y) * f64::cos(PI / 6.),
            (max_x - max_y) * f64::cos(PI / 6.),
        );

        Hexagon::new(min_x, min_y, max_x, max_y, min_h, max_h)
    }

    /// Indicates whether two Hexagons overlap.
    ///
    /// The test is based on the hyperplane separation theorem. The
    /// axis being check for intersection are:
    ///
    ///   - `x` in 3D space
    ///   - `y` in 3D space
    ///   - `x` in 2D space, called `h` for *horizontal*
    pub fn intersect(
        hex1: &Hexagon,
        pos1: &na::Vector3<f64>,
        hex2: &Hexagon,
        pos2: &na::Vector3<f64>,
    ) -> bool {
        let (x1, y1) = (pos1.x + pos1.z, pos1.y + pos1.z);
        let (x2, y2) = (pos2.x + pos2.z, pos2.y + pos2.z);

        let (min_x1, min_y1, min_h1) = (x1 + hex1.min_x, y1 + hex1.min_y, hex1.min_h);
        let (max_x1, max_y1, max_h1) = (x1 + hex1.max_x, y1 + hex1.max_y, hex1.max_h);
        let (min_x2, min_y2, min_h2) = (x2 + hex2.min_x, y2 + hex2.min_y, hex2.min_h);
        let (max_x2, max_y2, max_h2) = (x2 + hex2.max_x, y2 + hex2.max_y, hex2.max_h);

        // `x` axis intersection
        !(min_x1 >= max_x2 || min_x2 >= max_x1)

        // `y` axis intersection
        && !(min_y1 >= max_y2 || min_y2 >= max_y1)

        // `h` axis intersection
        && !(min_h1 >= max_h2 || min_h2 >= max_h1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use na::Vector3;

    #[test]
    fn test_intersect() {
        let hex1 = Hexagon::from_bounding_box(&BoundingBox::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(1., 1., 1.),
        ));
        let pos1 = Vector3::new(0., 0., 0.);

        let hex2 = Hexagon::from_bounding_box(&BoundingBox::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(1., 1., 1.),
        ));
        let pos2 = Vector3::new(0., 0., 0.);

        assert!(Hexagon::intersect(&hex1, &pos1, &hex2, &pos2), "Hexagons did not intersect");
    }
}
