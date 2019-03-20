use specs::prelude::*;

use crate::common::DeltaTime;
use crate::pathfinder::Pather;
use crate::position::Position;

const PROXIMITY: f64 = 0.1;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Actor {
    /// In 3D units
    walk_speed: f64,
}

impl Actor {
    pub fn new() -> Self {
        Actor { walk_speed: 1.0 }
    }
}

/// Moves actors
pub struct WalkerSystem;

impl WalkerSystem {
    pub fn new() -> Self {
        WalkerSystem
    }
}

impl<'a> System<'a> for WalkerSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Actor>,
        WriteStorage<'a, Pather>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, actors, mut pathers, mut positions): Self::SystemData) {
        use specs::Join;

        (&actors, &mut pathers, &mut positions)
            .join()
            // resetting all pathers would result in strange behaviour
            .filter(|(_actor, pather, _pos)| pather.has_path())
            .for_each(|(actor, pather, pos)| {
                if let Some(node) = pather.current() {
                    //println!("{:?}", node);
                    let target = na::Vector3::<f64>::new(
                        node.pos.x() as f64,
                        node.pos.y() as f64,
                        node.pos.z() as f64,
                    );
                    let diff = (target - pos.to_vector()).normalize();
                    let walk_vector = diff * actor.walk_speed * dt.0;
                    let new_pos = pos.to_vector() + walk_vector;
                    pos.set_x(new_pos.x);
                    pos.set_y(new_pos.y);
                    pos.set_z(new_pos.z);

                    if walk_vector.magnitude() <= PROXIMITY {
                        pather.next();
                    }
                } else {
                    pather.reset();
                }
            });
    }
}
