use specs::{Component, DenseVecStorage};
use std::convert::Infallible;

#[derive(Debug)]
#[repr(u32)]
#[allow(dead_code)]
pub enum Body {
    Point,
    Circle(f32),
    CircleOutline(f32, f32),
    Rectangle(f32, f32),
    RectangleOutline(f32, f32, f32),
    Line(vek::vec2::Vec2<f32>, f32),
    // TODO: Add.
    Triangle(Infallible),
    Polygon(Infallible),
}

impl Component for Body {
    type Storage = DenseVecStorage<Self>;
}
