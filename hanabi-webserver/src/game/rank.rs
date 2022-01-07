use serde::Deserialize;
use super::clue::ValidClueIdx;
use std::convert::TryFrom;
use super::GameSpecError;
use super::Cluelike;
use std::cmp::Ordering;
use snafu::Snafu;

#[derive(Debug, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
#[serde(try_from="i8")]
pub enum Rank {
    Cardinal(ValidClueIdx),
    Start,
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        match self {
            Start => { match other {
                Start => Ordering::Equal,
                _ => Ordering::Less,
            }},
            Cardinal(self_idx) => { match other{
                Start => Ordering::Greater,
                Cardinal(other_idx) => self_idx.cmp(other_idx),
            }}
        }
    }
}

impl TryFrom<i8> for Rank {
    type Error = GameSpecError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Ok(Rank::from_valid_clue_idx(ValidClueIdx::new(value)?))
    }
}

use Rank::*;

impl super::Cluelike for Rank {
    fn from_valid_clue_idx(idx: super::ValidClueIdx) -> Rank {
        let val = idx.val();
        match val {
            0 => Start,
            1..=std::i8::MAX => Cardinal(idx),
            _ => unreachable!("ValidClueIdx should always be zero or positive")
        }
    }
    fn to_idx(&self) -> ValidClueIdx {
        match *self {
            Start => ValidClueIdx::new(0).unwrap(),
            Cardinal(val) => val,
        }
    }
}
