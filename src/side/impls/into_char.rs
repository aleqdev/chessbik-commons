use super::*;

impl Into<char> for Side {
    fn into(self) -> char {
        match self {
            Side::TOP => 't',
            Side::LEFT => 'l',
            Side::FORWARD => 'f',
            Side::RIGHT => 'r',
            Side::BACK => 'b',
            Side::BOTTOM => 'm'
        }
    }
}

impl Into<char> for &Side {
    fn into(self) -> char {
        match self {
            Side::TOP => 't',
            Side::LEFT => 'l',
            Side::FORWARD => 'f',
            Side::RIGHT => 'r',
            Side::BACK => 'b',
            Side::BOTTOM => 'm'
        }
    }
}