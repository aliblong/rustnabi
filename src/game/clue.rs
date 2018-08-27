use super::*;

pub enum ClueType {
    Color,
    Rank,
}

pub struct Clue {
    pub ty: ClueType,
    pub val: Index,
}
