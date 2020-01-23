//! # Isometric Projection
//!
//! We have two different measurment units
//!
//! ## Unit
//!
//! Units represent the position in the Grid. The size of one grid
//! cell is one unit along each axis.
//!
//! Pixels
//!
//! ## Coordinate Spaces
//!
//! We have three different coordinate spaces
//!
//! Cartesian
//!
//! Isometric
//!
//! World
//!
//! Screen
//!
//! ## References
//!
//!   - [Isometric video game graphics](https://en.wikipedia.org/wiki/Isometric_video_game_graphics)
//!   - [Drawing isometric boxes in the correct order by Shaun Lebron](https://shaunlebron.github.io/IsometricBlocks/)
//!   - [Hyperplane separation theorem](https://en.wikipedia.org/wiki/Hyperplane_separation_theorem)
//!   - [Filmation math](http://bannalia.blogspot.com/2008/02/filmation-math.html)

mod bounding_box;
mod hexagon;
mod position;
mod projection;

pub use bounding_box::*;
pub use hexagon::*;
pub use position::*;
pub use projection::*;
