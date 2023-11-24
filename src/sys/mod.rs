use specs::DispatcherBuilder;

use self::physics_sim::PhysicsSimSys;

pub(self) mod borders;
pub(self) mod movement;
pub(crate) mod physics_sim;
pub(self) mod spawner;

// use macroquad::window::{screen_height, screen_width};

pub trait DispatcherBuilderExt {
    fn register_systems(self) -> Self;
}

impl<'a, 'b> DispatcherBuilderExt for DispatcherBuilder<'a, 'b> {
    fn register_systems(self) -> Self {
        self
            .with(PhysicsSimSys, "physics_sim", &[])
            .with(movement::MoveAndSyncSys, "move_and_sync", &["physics_sim"])
        //  .with(borders::BordersSys(screen_height(), screen_width()), "border_bounds", &[]).
    }
}
