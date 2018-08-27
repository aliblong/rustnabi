use super::*;

pub type PlayError = ();

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::<Card>::with_capacity(5) }
    }

    pub fn draw(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn send(&mut self, slot: Index) -> Result<Card, PlayError> {
        let slot = slot as usize;
        match slot < self.cards.len() {
            true => Ok(self.cards.remove(slot)),
            false => Err(()),
        }
    }
}
