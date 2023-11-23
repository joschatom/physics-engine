use specs::{System, ReadStorage, WriteStorage, Join as _};

use crate::comp::{Pos, Vel};

pub(crate) struct MovementSys;

impl<'a> System<'a> for MovementSys {
    // These are the resources required for execution.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.

        // This joins the component storages for Position
        // and Velocity together; it's also possible to do this
        // in parallel using rayon's `ParallelIterator`s.
        // See `ParJoin` for more.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}