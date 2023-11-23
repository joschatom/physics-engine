use std::cmp::Ordering;

use specs::{Component, DenseVecStorage};
use vek::Vec2;

pub struct ZIdx(pub i32);

impl Component for ZIdx{
    type Storage = DenseVecStorage::<Self>;
}