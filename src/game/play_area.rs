use super::*;

pub struct PlayArea {
    stacks: Vec<Index>
}

impl PlayArea {
    pub fn new(n_suits: Index) -> PlayArea {
        PlayArea { stacks: vec![0; n_suits] }
    }

    pub fn recv(&mut self, card: Card) -> Option<Card> {
        let play_stack = &mut self.stacks[card.suit];
        if *play_stack == card.rank - 1 {
            *play_stack += 1;
            return None;
        }
        else {
            return Some(card);
        }
    }
}
