use super::*;
use futures::future::{err, ok, FutureResult};
use tokio::prelude::*;

// Right now this is kind of pointless, but more things may be added to it later, like a WS
// connection?
pub struct Player {
    hand: Hand,
}

impl Player {
    fn draw(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.draw() {
            self.hand.draw(card);
        }
    }

    // TODO: refactor these two functions into one
    pub fn play(&mut self, slot: Index, board: &mut Board) -> Result<(), OoBError> {
        let card = self.hand.send(slot)?;
        board.play(card);
        Ok(())
    }
    pub fn discard(&mut self, slot: Index, board: &mut Board) -> Result<(), OoBError> {
        let card = self.hand.send(slot)?;
        board.discard(card);
        Ok(())
    }

    // Evaluates a clue against a hand.
    // Returns the indices of the cards touched by a clue.
    pub fn clue(&self, clue: Clue, hand: &Hand) -> Vec<Index> {
        hand.cards
            .iter()
            .enumerate()
            .filter(|(_card_index, card)| clue.matches(card))
            .map(|(card_index, _card)| card_index)
            .collect()
    }

    pub fn recv(&self, msg: String) {
        unimplemented!();
    }

    pub fn send(&self) -> FutureResult<&str, ()> {
        ok("hi")
    }
}
