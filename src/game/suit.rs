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

#[derive(Debug, Deserialize)]
struct Touches<ClueKind: Cluelike + std::fmt::Debug + Eq + std::hash::Hash>(TouchesMap<ClueKind>);

//impl<ClueKind: Cluelike> Touches<ClueKind> {
//    fn new(yaml_map: HashMap<i8, Vec<i8>>)
//        -> Result<Touches<ClueKind>, GameSpecError>
//    {
//        // Vec<Result> is supposedly converted to Result<Vec>
//        let map: TouchesMap<ClueKind> = yaml_map.into_iter()
//            .map(|(rank, touches)| (
//                    ValidClueIdx::new(rank)?.val(),
//                    touches.map(
//                        |touch|
//                        ClueKind::from_valid_clue_idx(ValidClueIdx::new(touch)?)
//                    ).collect()?
//                )
//            ).collect()?;
//        let ranks = map.ranks().collect();
//        let max = ranks.iter().rev().next().unwrap_or(-1);
//        if max == -1 {
//            return Err(InvalidTouches{kind: InvalidTouchesKind::Empty});
//        }
//        let min = ranks.iter().next().unwrap();
//        let size = ranks.len();
//        let play_order = if min == 0 {PlayOrder::UpOrDown} else {PlayOrder::Normal};
//        let well_formed =
//            (play_order == PlayOrder::UpOrDown && size == max + 1) ||
//            (play_order == PlayOrder::Normal && size == max);
//        match well_formed {
//            true => Ok((Touches{map}, play_order)),
//            false => Err(InvalidTouches{kind: InvalidTouchesKind::Gapped{ranks}}),
//        }
//    }
//    fn does_clue_touch_card(&self, clue: ClueKind, card_rank: Rank) -> bool {
//        let idx = card_rank.to_idx();
//        let err_msg =
//            "Request for touches from invalid rank clue index. \
//            Game spec should already have been validated";
//        self.map.get(idx).expect(err_msg).contains(clue.to_idx())
//    }
//}

pub enum CreateSuitError {
    Parse(serde_yaml::Error),
    Invalid(InvalidSuitError),
}

#[derive(Debug, Snafu)]
pub enum InvalidSuitError {
    #[snafu(display("Empty touches spec"))]
    EmptyRankTouches,
    EmptyColorTouches,
    #[snafu(display("Gapped rank touches spec: {:?}", ranks))]
    Gapped { ranks: Vec<Rank> },
    #[snafu(display("A suit with a start card and fewer than 2 ranks is ill-defined"))]
    TooFewRanksForStartCard,
    RankTouchesSizeMismatch { touches: Touches<Rank>, size: i8 },
    ColorTouchesSizeMismatch { touches: Touches<Color>, size: i8 },
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
        suit.validate()
    }
    fn validate(self) -> Result<Self, CreateSuitError> {
        // not exactly optimal to not grab count, min, and max in one loop, but
        // perf cost seems negligible
        let rank_touches_ranks_count = self.rank_touches.0.keys().count();
        if rank_touches_ranks_count == 0 {
            return Err(CreateSuitError::Invalid(InvalidSuitError::EmptyRankTouches))
        }
        // unwrapping these is only safe with the previous check for nonzero size
        let rank_touches_ranks_min = self.rank_touches.0.keys().min().unwrap();
        let max_rank_val = self.rank_touches.0.keys().max().unwrap().to_idx().val();
        match rank_touches_ranks_min {
            Rank::Start => {
                if rank_touches_ranks_count < 3 {
                    Err(CreateSuitError::Invalid(InvalidSuitError::TooFewRanksForStartCard))
                }
                else if max_rank_val as usize != rank_touches_ranks_count - 1 {
                    Err(CreateSuitError::Invalid(InvalidSuitError::Gapped{
                        ranks: self.sorted_ranks_from_rank_touches()
                    }))
                }
                else {
                    Ok(self)
                }
            },
            Rank::Cardinal(_) => {
                if max_rank_val as usize != rank_touches_ranks_count {
                    Err(CreateSuitError::Invalid(InvalidSuitError::Gapped{
                        ranks: self.sorted_ranks_from_rank_touches()
                    }))
                }
                else {
                    Ok(self)
                }
            }
        }
    }
    fn sorted_ranks_from_rank_touches(self) -> Vec<Rank> {
        let mut ranks = self.rank_touches.0.keys()
            .map(|rank| rank.clone())
            .collect::<Vec<Rank>>();
        ranks.sort();
        ranks
    }
}

#[test]
fn rank_touches_deserialize_test() {
    let yaml_str: &'static str = indoc!(
        "---
        0:
            - 1
            - 2
        1:
            - 1
            - 2
        2:
            - 0
    "
    );
    let touches: TouchesMap<Rank> = serde_yaml::from_str(yaml_str).expect("Bad yaml");
    println!("{:?}", touches);
    println!("{}", yaml_str);
    //let mut vm = vec_map::VecMap::new();
    //vm.insert(1, "test");
    //println!("{}", serde_yaml::to_string(&vm).unwrap());
}

#[test]
fn suit_deserialize_test() {
    let yaml_str = indoc!(
        "---
        play_order: UpOrDown
        size: 2
        rank_touches:
            0:
                - 1
                - 2
            1:
                - 1
                - 2
            2:
                - 0
        color_touches:
            0:
                - 1
                - 5
            1:
                - 3
                - -2
            2:
                - 0
    "
    );
    let suit: serde_yaml::Result<Suit> = serde_yaml::from_str(yaml_str);
    println!("{:?}", suit);
}
