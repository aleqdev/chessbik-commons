use chessbik_board::{Piece, Side};

pub mod impls;
pub use impls::*;

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(Clone, Copy)]
    pub struct Cell {
        #[deref]
        #[deref_mut]
        pub piece: Option<Piece>,
        pub side: Side,
    }
);
