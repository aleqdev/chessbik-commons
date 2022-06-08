use chessbik_board::{Side, Piece};

pub mod impls;
pub use impls::*;

chessbik_commons::derive_wrapper!(
    #[derive(Clone, Copy)]
    pub struct Cell {
        #[deref]
        #[deref_mut]
        pub piece: Option<Piece>,
        pub side: Side,
    }
);
