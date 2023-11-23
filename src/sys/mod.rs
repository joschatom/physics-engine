use specs::DispatcherBuilder;

pub(self) mod movement;
pub(self) mod borders;

// use macroquad::window::{screen_height, screen_width};

pub trait DispatcherBuilderExt {
    fn register_systems(self) -> Self;
}

impl<'a, 'b> DispatcherBuilderExt for DispatcherBuilder<'a, 'b>{
    fn register_systems(self) -> Self {
        self
        .with(movement::MovementSys, "movement", &[])
        // .with(borders::BordersSys(screen_height(), screen_width()), "border_bounds", &[])
    }
}