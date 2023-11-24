use specs::{Component, DenseVecStorage};

#[derive(Debug, Default)]
pub struct Mass(pub(crate) f32);

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}
