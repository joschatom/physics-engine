use specs::{Builder, Component, DenseVecStorage};
use vek::Vec2;

#[derive(Debug, Default)]
pub struct Pos(pub Vec2<f32>);

impl Component for Pos {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct PrevPos(pub Vec2<f32>);

impl Component for PrevPos {
    type Storage = DenseVecStorage<Self>;
}
