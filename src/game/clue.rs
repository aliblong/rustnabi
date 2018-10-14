use super::*;

pub enum Clue {
    Color(Index, ColorSuitMap),
    Rank(Index),
}

impl Clue {
    pub fn matches(&self, card: &Card) -> bool {
        match self {
            Clue::Color(val, map) => map[*val].contains(&card.suit),
            Clue::Rank(val) => card.suit == *val,
        }
    }
}
