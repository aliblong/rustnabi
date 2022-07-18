use serde::Deserialize;
use std::convert::TryFrom;
use std::collections::HashSet;
use super::GameSpecError;

pub type Properties = Vec<HashSet<PropertyIndex>>;

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[serde(try_from="i8")]
pub struct PropertyIndex(i8);

impl TryFrom<i8> for PropertyIndex {
    type Error = GameSpecError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        PropertyIndex::new(value)
    }
}

impl PropertyIndex {
    pub fn new(val: i8) -> Result<Self, GameSpecError> {
        match val {
            0..=std::i8::MAX => Ok(Self(val)),
            _ => Err(GameSpecError::InvalidClueIdx {val})
        }
    }
    pub fn val(&self) -> i8 { self.0 }
}
