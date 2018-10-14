use super::*;

pub type OoBError = ();

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::<Card>::with_capacity(4) }
    }

    pub fn draw(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn send(&mut self, slot: Index) -> Result<Card, OoBError> {
        let slot = slot as usize;
        match slot < self.cards.len() {
            true => Ok(self.cards.remove(slot)),
            false => Err(()),
        }
    }
}
