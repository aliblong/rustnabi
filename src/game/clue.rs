use super::*;
use serde::Deserialize;
use std::convert::TryFrom;
use super::GameSpecError;
//use std::fmt;

//pub enum Clue {
//    Color(Index, ColorSuitMap),
//    Rank(Index),
//}
//
//impl Clue {
//    pub fn matches(&self, card: &Card) -> bool {
//        match self {
//            Clue::Color(val, map) => map[*val].contains(&card.suit),
//            Clue::Rank(val) => card.suit == *val,
//        }
//    }
//}

pub trait Cluelike {
    fn to_idx(&self) -> ValidClueIdx;
    fn from_valid_clue_idx(idx: ValidClueIdx) -> Self;
}

// can't use a generic impl like this without a specialization feature in Rust
//impl<T> TryFrom<i8> for T
//where T: Cluelike
//{
//    type Error = GameSpecError;
//    fn try_from(value: i8) -> Result<Self, Self::Error> {
//        Ok(T::from_valid_clue_idx(ValidClueIdx::new(value)?))
//    }
//}


#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[serde(try_from="i8")]
pub struct ValidClueIdx(i8);

impl TryFrom<i8> for ValidClueIdx {
    type Error = GameSpecError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        ValidClueIdx::new(value)
    }
}

//impl fmt::Display for ValidClueIdx {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.val())
//    }
//}

impl ValidClueIdx {
    pub fn new(val: i8) -> Result<Self, GameSpecError> {
        match val {
            0..=std::i8::MAX => Ok(Self(val)),
            _ => Err(GameSpecError::InvalidClueIdx {val})
        }
    }
    pub fn val(&self) -> i8 { self.0 }
}
