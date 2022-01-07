use super::*;
use serde::Deserialize;
use std::convert::TryFrom;
use super::GameSpecError;

pub trait Cluelike {
    fn to_idx(&self) -> ValidClueIdx;
    fn from_valid_clue_idx(idx: ValidClueIdx) -> Self;
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[serde(try_from="i8")]
pub struct ValidClueIdx(i8);

impl TryFrom<i8> for ValidClueIdx {
    type Error = GameSpecError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        ValidClueIdx::new(value)
    }
}

impl ValidClueIdx {
    pub fn new(val: i8) -> Result<Self, GameSpecError> {
        match val {
            0..=std::i8::MAX => Ok(Self(val)),
            _ => Err(GameSpecError::InvalidClueIdx {val})
        }
    }
    pub fn val(&self) -> i8 { self.0 }
}
