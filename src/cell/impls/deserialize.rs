use crate::SideVisitor;

use super::*;

use chessbik_board::PieceVisitor;
use serde::{de::{Error, Visitor}, Deserialize, Deserializer};

impl<'de> Deserialize<'de> for Cell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        let s = String::deserialize(deserializer)?;
        let mut s = s.chars();

        Ok(Self {
            piece: match s.next() {
                Some('-') => None,
                Some(x) => Some(PieceVisitor.visit_char(x)?),
                None => return Err(D::Error::invalid_length(0, &"2"))
            },
            side: match s.next() {
                Some(x) => SideVisitor.visit_char(x)?,
                None => return Err(D::Error::invalid_length(0, &"2"))
            }
        })
    }
}
