use std::cmp::min;

use specs::{
    rayon::iter::ParallelIterator as _, Join as _, ParJoin as _, ReadStorage, System, WriteStorage,
};
use vek::{partial_min, Vec2};

use crate::comp::{Pos, PrevPos, SimVel, Vel};

pub(crate) struct MoveAndSyncSys;

impl<'a> System<'a> for MoveAndSyncSys {
    // These are the resources required for execution.
    type SystemData = (
        WriteStorage<'a, Pos>,
        WriteStorage<'a, PrevPos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, SimVel>,
    );

    fn run(&mut self, (mut pos, mut prev_pos, mut vel, mut sim_vel): Self::SystemData) {
        (&mut pos, &mut prev_pos, &mut vel, &mut sim_vel)
            .par_join()
            .for_each(|(pos, prev_pos, vel, sim_vel)| {
                if (vel.0.y < 0.0) {
                    println!("SYNC {:?} <=> {:?}", vel.0, sim_vel.0);
                } else {
                }
                sim_vel.0 *= crate::consts::PHYSICS_DELTA;
                vel.0 += sim_vel.0;
                sim_vel.0 = Vec2::<f32>::zero();
                pos.0 += vel.0;

                if pos.0.y > 1000.0 {
                    pos.0.y = macroquad::rand::gen_range(20.0, 800.0) - pos.0.y;
                    vel.0 = Vec2::<f32>::zero();
                }
                if pos.0.x > 2000.0 {
                    pos.0.y = macroquad::rand::gen_range(20.0, 800.0);
                    vel.0 = Vec2::<f32>::zero();
                }
            });
    }
}
