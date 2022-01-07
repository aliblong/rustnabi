use serde::Deserialize;
use super::*;
use std::convert::TryFrom;
use super::GameSpecError;
use super::Cluelike;

#[derive(Debug, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
#[serde(try_from="i8")]
pub struct Color(ValidClueIdx);

impl TryFrom<i8> for Color {
    type Error = GameSpecError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Color::from_valid_clue_idx(ValidClueIdx::new(value)?))
    }
}

impl super::Cluelike for Color {
    fn from_valid_clue_idx(idx: super::ValidClueIdx) -> Self {
        Color(idx)
    }
    fn to_idx(&self) -> ValidClueIdx {
        self.0
    }
}
