use specs::{Component, DenseVecStorage};
use vek::Vec2;

pub struct Vel(pub Vec2::<f32>);

impl Component for Vel{
    type Storage = DenseVecStorage::<Self>;
}