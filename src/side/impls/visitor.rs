use serde::de::{Error, Visitor, Unexpected};

use super::*;

pub struct SideVisitor;

impl<'de> Visitor<'de> for SideVisitor {
    type Value = Side;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(EXPECTING)
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error, 
    {
        match Side::try_from(v) {
            Ok(v) => Ok(v),
            Err(err) => Err(E::invalid_value(Unexpected::Char(v), &err))
        }
    }
}