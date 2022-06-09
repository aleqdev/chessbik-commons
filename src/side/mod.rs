pub mod impls;
pub use impls::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    TOP,
    LEFT,
    FORWARD,
    RIGHT,
    BACK,
    BOTTOM,
}
