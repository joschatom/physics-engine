use specs::{Component, DenseVecStorage};
use vek::Vec2;

pub struct Pos(pub Vec2::<f32>);

impl Component for Pos{
    type Storage = DenseVecStorage::<Self>;
}