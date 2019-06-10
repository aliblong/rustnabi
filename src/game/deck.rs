use super::card::Card;
use super::variant::Variant;
use crate::db::models::Index;
use hash::hash;
use rand::prng::XorShiftRng;
use rand::{Rng, SeedableRng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(variant: &Variant) -> Deck {
        let mut cards = Vec::<Card>::with_capacity(50);
        for (suit_index, suit) in variant.suits.iter().enumerate() {
            for (rank_minus_one, multiplicity) in suit.dist.iter().enumerate() {
                cards.push(Card {
                    suit: suit_index as Index,
                    rank: (rank_minus_one + 1) as Index,
                });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self, seed_str: &str) {
        // Seeds in for XorShift are of type [u8; 16]
        let hashed_seed = hash(seed_str.as_bytes());
        let seed: [u8; 16] = seed_from_hash(hashed_seed.as_slice());
        let mut rng = XorShiftRng::from_seed(seed);
        rng.shuffle(self.cards.as_mut_slice());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

// ONLY call this with output from hash, which will be more than 16 bytes long.
fn seed_from_hash(hash: &[u8]) -> [u8; 16] {
    // Need to either default initialize or run unsafe code, and the former seems like a
    // negligible perf cost.
    let mut res: [u8; 16] = Default::default();
    let hash = &hash[..16]; // panics if not enough data
    res.copy_from_slice(hash);
    res
}

#[cfg(test)]
mod test {
    use super::super::variant::Variant;
    use super::Deck;
    use test::*;

    #[test]
    fn normal_variant() {
        let variant = Variant::new(NORMAL_VARIANT);
        let mut deck = Deck::new(&variant);
        deck.shuffle("carducci_game");
        panic!("{:?}", deck);
    }
}
