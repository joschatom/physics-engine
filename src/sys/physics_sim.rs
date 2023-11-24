use specs::{Join as _, ReadStorage, System, WriteStorage};
use vek::Vec2;

use crate::comp::{Mass, Pos, PrevPos, SimVel};

pub(crate) struct PhysicsSimSys;

impl<'a> System<'a> for PhysicsSimSys {
    // These are the resources required for execution.
    type SystemData = (WriteStorage<'a, SimVel>, ReadStorage<'a, Mass>);

    fn run(&mut self, (mut vel, mass): Self::SystemData) {
        for (vel, mass) in (&mut vel, &mass).join() {
            let gravitation_force = mass.0 * Vec2::new(0.0, -9.81);
            let external_forces = gravitation_force;
            vel.0 -= external_forces;
        }
    }
}
