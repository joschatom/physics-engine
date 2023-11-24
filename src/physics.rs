use std::convert::Infallible;

pub enum Physics {
    Default,
    Custom(Infallible),
}
