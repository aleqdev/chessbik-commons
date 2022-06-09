use serde::{Serialize, Serializer};

use super::*;

impl Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer 
    {
        serializer.serialize_char(self.into())
    }
}