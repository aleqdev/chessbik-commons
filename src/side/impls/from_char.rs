use super::*;

pub const EXPECTING: &str = "(t, l, f, r, b, m)";

impl TryFrom<char> for Side {
    type Error = &'static str;
    
    fn try_from(c: char) -> Result<Side, Self::Error> {
        match c {
            't' => Ok(Side::TOP),
            'l' => Ok(Side::LEFT),
            'f' => Ok(Side::FORWARD),
            'r' => Ok(Side::RIGHT),
            'b' => Ok(Side::BACK),
            'm' => Ok(Side::BOTTOM),
            _ => Err(EXPECTING)
        }
    }
}
