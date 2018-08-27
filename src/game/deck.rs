use super::card::Card;
use super::variant::Variant;
use db::models::Index;
use rand::{Rng, SeedableRng, ChaChaRng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(variant: &Variant) -> Deck {
        let mut cards = Vec::<Card>::with_capacity(50);
        for (suit_index, suit) in variant.suits.iter().enumerate() {
            for (rank_minus_one, multiplicity) in suit.dist.iter().enumerate() {
                cards.push(Card { suit: suit_index as Index, rank: (rank_minus_one + 1) as Index});
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self, seed: &str) {
        // Seeds in `rand` are of type Vec<u32>
        let seed = seed.bytes().map(|byte| byte as u32).collect::<Vec<u32>>();
        let mut rng = ChaChaRng::from_seed(&seed);
        rng.shuffle(self.cards.as_mut_slice());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[cfg(test)]
mod test {
    use test::*;
    use super::super::variant::Variant;
    use super::Deck;

    #[test]
    fn normal_variant() {
        let variant = Variant::new(NORMAL_VARIANT);
        let mut deck = Deck::new(&variant);
        deck.shuffle("carducci_game");
        panic!("{:?}", deck);
    }
}
