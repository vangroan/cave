/// Total width of tile on screen
pub const TILE_WIDTH_2D: f64 = 80.;

/// Total height of tile on screen
pub const TILE_HEIGHT_2D: f64 = 80.;

/// Partial deepness of tile downwards, on screen
pub const TILE_DEPTH_2D: f64 = 50.;

pub const HALF_TILE_3D: f64 = 0.5;

pub fn flatten_pos(iso_pos: &na::Vector3<f64>) -> na::Vector2<f64> {
    let x = iso_pos.x * TILE_WIDTH_2D;
    let y = iso_pos.y * TILE_HEIGHT_2D;
    let z = iso_pos.z * TILE_DEPTH_2D;

    na::Vector2::<f64>::new(x, y - z)
}
