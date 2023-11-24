use specs::{Join as _, ReadStorage, System, WriteStorage, Entities, Write, world::EntitiesRes};
use vek::Vec2;

use crate::comp::{Mass, Pos, PrevPos, SimVel, vel};

pub(crate) struct EntitySpawnerSys;

impl<'a> System<'a> for EntitySpawnerSys {
    // These are the resources required for execution.
    type SystemData = Write<'a, EntitiesRes>;

    fn run(&mut self, (entities): Self::SystemData) {
       
    }
}
