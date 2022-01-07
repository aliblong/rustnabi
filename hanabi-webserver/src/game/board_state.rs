use super::*;

pub struct BoardState {
    pub play_area: PlayArea,
    pub discard_area: DiscardArea,
    pub hands: Vec<Hand>,
    pub deck: Deck,
    pub clues: i8,
    pub strikes: i8,
    pub current_player: i8,
}

impl BoardState {
    pub fn new(variant: &Variant) -> BoardState {
        BoardState {
            play_area: PlayArea::new(variant.suits.len()),
            discard_area: DiscardArea {},
            deck: Deck::new(variant),
            clues: MAX_CLUES,
            strikes: 0,
        }
    }

    pub fn play(&mut self, card: Card) {
        let rank = card.rank;
        if let Some(card) = self.play_area.recv(card) {
            self.discard_area.recv(card);
            self.strikes += 1;
        }
        // There's no way yet to specify which type of plays should modify clues and how, so
        // hardcode in that 5s and above will return a clue.
        else if rank >= 5 && self.clues < MAX_CLUES {
            self.clues += 1;
        }
    }

    pub fn discard(&mut self, card: Card) {
        self.discard_area.recv(card);
        // In the base rules, players aren't allowed to discard at max clues, so don't bother
        // checking if it will go over max clues.
        self.clues += 1;
    }
}
