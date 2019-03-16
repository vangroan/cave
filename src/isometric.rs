//! Isometric Projection
//! 
//! Isomtric tile dimensions:
//! 
//!  _____width_____
//! |               |
//!                  _
//!      _ * * _      |
//!   _ *       * _   |
//!  <_           _>  | height
//!  |  *_     _*  | _|   _ 
//!  |     * *     |       |           
//!  |      |      |       |    
//!  *_     |     _*       | depth
//!     *_  |  _*          |
//!        *|*            _|
//!

pub struct Isometric {
    tile_size: na::Vector3<u32>,
}

impl Isometric {
    pub fn new(tile_width: u32, tile_height: u32, tile_depth: u32) -> Self {
        Isometric {
            tile_size: na::Vector3::new(tile_width, tile_height, tile_depth),
        }
    }

    /// Convert 3D Cartesian coordinates to 2D Isometric Coordinates
    pub fn cart_to_iso<N>(pos: na::Vector3<N>) -> na::Vector3<N>
    where
        N : na::Scalar + nt::Num + nt::NumCast,
    {
        let _2 : N = nt::NumCast::from(2).unwrap();
        let _4 : N = nt::NumCast::from(4).unwrap();
        let x = (pos.x - pos.y) / _2;
        let y = (pos.x + pos.y) / _4;
        na::Vector3::new(x, y, pos.z)
    }

    pub fn iso_to_cart<N>(pos: na::Vector3<N>) -> na::Vector3<N>
    where
        N : na::Scalar + nt::Num + nt::NumCast,
    {
        let _2 : N = nt::NumCast::from(2).unwrap();
        let _4 : N = nt::NumCast::from(4).unwrap();
        let x = (pos.x * _2 + pos.y * _4) / _2;
        let y = pos.y * _4 - x;
        na::Vector3::new(x, y, pos.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use na::Vector3;

    #[test]
    fn test_cart_to_iso() {
        // Four corners of a tile
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), Isometric::cart_to_iso(Vector3::<f32>::new(0., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(0.5, 0.25, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(1., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(0., 0.5, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(1., 1., 0.)));
        assert_eq!(Vector3::<f32>::new(-0.5, 0.25, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(0., 1., 0.)));

        // Tile in negative coordinates
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), Isometric::cart_to_iso(Vector3::<f32>::new(0., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(-0.5, -0.25, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(-1., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(0., -0.5, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(-1., -1., 0.)));
        assert_eq!(Vector3::<f32>::new(0.5, -0.25, 0.), Isometric::cart_to_iso(Vector3::<f32>::new(0., -1., 0.)));
    }

    #[test]
    fn test_iso_to_cart() {
        // Four corners of a tile
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(1., 0., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0.5, 0.25, 0.)));
        assert_eq!(Vector3::<f32>::new(1., 1., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0., 0.5, 0.)));
        assert_eq!(Vector3::<f32>::new(0., 1., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(-0.5, 0.25, 0.)));

        // Tile in negative coordinates
        assert_eq!(Vector3::<f32>::new(0., 0., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0., 0., 0.)));
        assert_eq!(Vector3::<f32>::new(-1., 0., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(-0.5, -0.25, 0.)));
        assert_eq!(Vector3::<f32>::new(-1., -1., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0., -0.5, 0.)));
        assert_eq!(Vector3::<f32>::new(0., -1., 0.), Isometric::iso_to_cart(Vector3::<f32>::new(0.5, -0.25, 0.)));
    }
}
