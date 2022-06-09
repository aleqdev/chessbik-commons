use chessbik_board::Piece;

pub mod impls;
pub use impls::*;

use crate::Side;

chessbik_derive_wrapper::derive_wrapper!(
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Cell {
        #[deref]
        #[deref_mut]
        pub piece: Option<Piece>,
        pub side: Side,
    }
);
