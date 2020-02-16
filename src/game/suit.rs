use super::{Cluelike, Color, Rank, ValidClueIdx};
use super::{GameSpecError};
use indoc::indoc;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use snafu::Snafu;
// vec_map doesn't have a nice out-of-the-box serde impl
//use vec_map;

#[derive(Debug, Deserialize)]
enum PlayOrder {
    Normal,
    UpOrDown,
}

// A given suit's "touches" refers to which color and rank clues touch which
// ranks of cards.
// For most variants, the suits will have a 1:1 mapping of rank clues to ranks,
// and suits will have either 1, 2, or all (rainbow) colors.
type TouchesMap<ClueKind> = HashMap<Rank, HashSet<ClueKind>>;

#[derive(Debug, Deserialize, PartialEq)]
struct Touches<ClueKind: Cluelike + std::fmt::Debug + Eq + std::hash::Hash>(TouchesMap<ClueKind>);

impl<ClueKind: Cluelike + std::fmt::Debug + Eq + std::hash::Hash> Touches<ClueKind> {
    fn validate(&self) -> Result<(), InvalidTouchesError> {
        // not exactly optimal to not grab count, min, and max in one loop, but
        // perf cost seems negligible
        let rank_touches_ranks_count = self.0.keys().count();
        if rank_touches_ranks_count == 0 {
            return Err(InvalidTouchesError::Empty)
        }
        // unwrapping these is only safe with the previous check for nonzero size
        let rank_touches_ranks_min = self.0.keys().min().unwrap();
        let max_rank_val = self.0.keys().max().unwrap().to_idx().val();
        match rank_touches_ranks_min {
            Rank::Start => {
                if rank_touches_ranks_count < 3 {
                    Err(InvalidTouchesError::TooFewRanksForStartCard)
                }
                else if max_rank_val as usize != rank_touches_ranks_count - 1 {
                    Err(InvalidTouchesError::Gapped{
                        ranks: self.sorted_ranks()
                    })
                }
                else {
                    Ok(())
                }
            },
            Rank::Cardinal(_) => {
                if max_rank_val as usize != rank_touches_ranks_count {
                    Err(InvalidTouchesError::Gapped{
                        ranks: self.sorted_ranks()
                    })
                }
                else {
                    Ok(())
                }
            }
        }
    }
    fn sorted_ranks(&self) -> Vec<Rank> {
        let mut ranks = self.0.keys()
            .map(|rank| rank.clone())
            .collect::<Vec<Rank>>();
        ranks.sort();
        ranks
    }
}

#[derive(Debug, Snafu)]
pub enum InvalidTouchesError {
    #[snafu(display("Empty touches spec"))]
    Empty,
    #[snafu(display("Gapped touches spec: {:?}", ranks))]
    Gapped { ranks: Vec<Rank> },
    #[snafu(display("A suit with a start card and fewer than 2 ranks is ill-defined"))]
    TooFewRanksForStartCard,
}


#[derive(Debug)]
pub enum CreateSuitError {
    Parse(serde_yaml::Error),
    Invalid(InvalidSuitError),
}

#[derive(Debug, Snafu)]
pub enum InvalidSuitError {
    InvalidRankTouches { err: InvalidTouchesError },
    InvalidColorTouches { err: InvalidTouchesError },
    TouchesSizeMismatch { ranks: Vec<Rank>, size: i8 },
    PlayOrderTouchesMismatch { err: PlayOrderTouchesMismatchError },
}

#[derive(Debug, Snafu)]
pub enum PlayOrderTouchesMismatchError {
    PlayOrderNormalTouchesUpOrDown,
    PlayOrderUpOrDownTouchesNormal,
}

#[derive(Debug, Deserialize)]
pub struct Suit {
    size: ValidClueIdx,
    rank_touches: Touches<Rank>,
    color_touches: Touches<Color>,
    play_order: PlayOrder,
}

//impl From<serde::Error> for GameSpecError {
//    fn from(error: serde:Error) -> Self {
//    }
//}
//
impl Suit {
    pub fn new(yaml_str: &str) -> Result<Self, CreateSuitError> {
        let suit: Self = match serde_yaml::from_str(yaml_str) {
            Ok(suit) => suit,
            Err(err) => return Err(CreateSuitError::Parse(err)),
        };
        match suit.validate() {
            Ok(_) => Ok(suit),
            Err(err) => Err(CreateSuitError::Invalid(err)),
        }
    }
    fn validate(&self) -> Result<(), InvalidSuitError> {
        match self.rank_touches.validate() {
            Ok(_) => (),
            Err(err) => { return Err(InvalidSuitError::InvalidRankTouches { err }); },
        };
        match self.color_touches.validate() {
            Ok(_) => (),
            Err(err) => { return Err(InvalidSuitError::InvalidColorTouches { err }); },
        };
        let n_ranks = self.size.val();
        let n_ranks_rank_touches = self.rank_touches.0.len();
        let n_ranks_color_touches = self.color_touches.0.len();
        let rank_touches_sorted_ranks = self.rank_touches.sorted_ranks();
        match rank_touches_sorted_ranks.iter().min().unwrap() {
            Rank::Start => {
                match self.play_order {
                    PlayOrder::Normal => { return Err(InvalidSuitError::PlayOrderTouchesMismatch {
                        err: PlayOrderTouchesMismatchError::PlayOrderNormalTouchesUpOrDown
                    }) },
                    PlayOrder::UpOrDown => (),
                }
            },
            Rank::Cardinal(_) => { // guaranteed to be value 1
                match self.play_order {
                    PlayOrder::Normal => (),
                    PlayOrder::UpOrDown => { return Err(InvalidSuitError::PlayOrderTouchesMismatch {
                        err: PlayOrderTouchesMismatchError::PlayOrderUpOrDownTouchesNormal
                    }) },
                }
            }
        }
        let color_touches_sorted_ranks = self.color_touches.sorted_ranks();
        if  n_ranks as usize != n_ranks_rank_touches {
            return Err(InvalidSuitError::TouchesSizeMismatch {
                ranks: rank_touches_sorted_ranks,
                size: n_ranks,
            });
        }
        if  n_ranks as usize != n_ranks_color_touches {
            return Err(InvalidSuitError::TouchesSizeMismatch {
                ranks: color_touches_sorted_ranks,
                size: n_ranks,
            });
        }
        return Ok(());
    }
}

#[test]
fn suit_deserialize_gapped_color_touches_test() {
    let yaml_str = indoc!(
        "---
        play_order: Normal
        size: 2
        rank_touches:
            1:
                - 1
                - 2
            2:
                - 0
        color_touches:
            1:
                - 3
                - 2
            3:
                - 0
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::InvalidColorTouches{
            err: InvalidTouchesError::Gapped{ranks: _}
        })
    );
}

#[test]
fn suit_deserialize_gapped_rank_touches_test() {
    let yaml_str = indoc!(
        "---
        play_order: Normal
        size: 2
        rank_touches:
            1:
                - 1
                - 2
            3:
                - 0
        color_touches:
            1:
                - 3
                - 2
            2:
                - 0
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::InvalidRankTouches{
            err: InvalidTouchesError::Gapped{ranks: _}
        })
    );
}

#[test]
fn suit_deserialize_rank_touches_size_mismatch_test() {
    let yaml_str = indoc!(
        "---
        play_order: Normal
        size: 3
        rank_touches:
            1:
                - 1
                - 2
            2:
                - 0
        color_touches:
            1:
                - 3
                - 2
            2:
                - 0
            3:
                - 0
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::TouchesSizeMismatch { ranks: _, size: _ })
    );
}

#[test]
fn suit_deserialize_color_touches_size_mismatch_test() {
    let yaml_str = indoc!(
        "---
        play_order: UpOrDown
        size: 3
        rank_touches:
            0:
                - 1
            1:
                - 1
                - 2
            2:
                - 0
            3:
                - 0
        color_touches:
            0:
                - 1
            1:
                - 3
                - 2
            2:
                - 0
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::TouchesSizeMismatch { ranks: _, size: _ })
    );
}

#[test]
fn suit_deserialize_too_few_ranks_start_card_test() {
    let yaml_str = indoc!(
        "---
        play_order: UpOrDown
        size: 1
        rank_touches:
            0:
                - 1
            1:
                - 1
                - 2
        color_touches:
            0:
                - 1
            1:
                - 1
                - 2
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::InvalidRankTouches {
            err: InvalidTouchesError::TooFewRanksForStartCard
        })
    );
}

#[test]
fn suit_deserialize_play_order_normal_touches_up_or_down_test() {
    let yaml_str = indoc!(
        "---
        play_order: Normal
        size: 2
        rank_touches:
            0:
                - 1
            1:
                - 1
                - 2
            2:
                - 1
        color_touches:
            0:
                - 1
            1:
                - 1
                - 2
            2:
                - 1
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::PlayOrderTouchesMismatch {
            err: PlayOrderTouchesMismatchError::PlayOrderNormalTouchesUpOrDown
        })
    );
}

#[test]
fn suit_deserialize_play_order_up_or_down_touches_normal_test() {
    let yaml_str = indoc!(
        "---
        play_order: UpOrDown
        size: 2
        rank_touches:
            1:
                - 1
                - 2
            2:
                - 1
        color_touches:
            1:
                - 1
                - 2
            2:
                - 1
    "
    );
    let suit = Suit::new(yaml_str);
    println!("{:?}", suit);
    matches!(
        suit.unwrap_err(),
        CreateSuitError::Invalid(InvalidSuitError::PlayOrderTouchesMismatch {
            err: PlayOrderTouchesMismatchError::PlayOrderUpOrDownTouchesNormal
        })
    );
}
