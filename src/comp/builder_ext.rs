use specs::{Builder, EntityBuilder};
use vek::Vec2;

pub trait EntityBuilderExt
where
    Self: Builder,
{
    fn with_position(self, pos: Vec2<f32>) -> Self;
    fn with_physics(self, _phys: crate::physics::Physics) -> Self;
}

impl<'a> EntityBuilderExt for EntityBuilder<'a> {
    fn with_position(self, pos: Vec2<f32>) -> Self {
        self.with(super::Pos(pos)).with(super::PrevPos(
            pos - (super::vel::Vel::default().0 * crate::consts::PHYSICS_DELTA),
        ))
    }
    fn with_physics(self, _phys: crate::physics::Physics) -> Self {
        self.with(super::Mass(1.0))
            .with(super::Vel(vek::Vec2::zero()))
            .with(super::SimVel(vek::Vec2::zero()))
    }
}
