use specs::{Component, DenseVecStorage};
use vek::Vec2;

#[derive(Debug, Default)]
pub struct Vel(pub Vec2<f32>);

impl Component for Vel {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct SimVel(pub Vec2<f32>);

impl Component for SimVel {
    type Storage = DenseVecStorage<Self>;
}
