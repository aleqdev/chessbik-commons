use chessbik_board::BoardTransform;

use super::*;

impl BoardTransform for Cell {
    fn by_slide(&mut self, other: &Self) {
        self.piece = other.piece;
    }

    fn by_rotation(&mut self, other: &Self) {
        *self = *other;
    }

    fn remove(&mut self) {
        self.piece = None;
    }
}