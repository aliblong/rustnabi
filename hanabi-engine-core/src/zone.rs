use super::{Card};

//#[derive(Debug)]
pub struct Zone {
    zones: Vec<Zone>,
    cards: Vec<Card>,
    fit: fn(&Zone, &Card) -> bool,
}
