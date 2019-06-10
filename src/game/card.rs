use crate::db::models::Index;

#[derive(Debug)]
pub struct Card {
    pub suit: Index,
    pub rank: Index,
}
