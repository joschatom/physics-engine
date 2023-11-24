use specs::{world::EntitiesRes, Entities, Join as _, ReadStorage, System, Write, WriteStorage};
use vek::Vec2;

use crate::comp::{vel, Mass, Pos, PrevPos, SimVel};

pub(crate) struct EntitySpawnerSys;

impl<'a> System<'a> for EntitySpawnerSys {
    // These are the resources required for execution.
    type SystemData = Write<'a, EntitiesRes>;

    fn run(&mut self, _entities: Self::SystemData) {}
}
