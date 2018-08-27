use super::*;

// Right now this is kind of pointless, but more things may be added to it later, like WS
// connections?
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
    pub fn play(&mut self, slot: Index, board: &mut Board) -> Result<(), PlayError> {
        let card = self.hand.send(slot)?;
        board.play(card);
        Ok(())
    }
    pub fn discard(&mut self, slot: Index, board: &mut Board) -> Result<(), PlayError> {
        let card = self.hand.send(slot)?;
        board.discard(card);
        Ok(())
    }
    /*
    pub fn clue(self, clue: Clue, hand: &Hand) {
        let card_matches_clue = |card| {
            match clue.ty {
                ClueType::Color => card.suit == clue.val,
                ClueType::Rank => card.suit == clue.val,
            }
        }
        hand.cards.iter().map(
    }
    */
}
