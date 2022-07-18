use super::{Properties, Card};

#[derive(Debug)]
pub struct Clue {
    not_disjoint: Option<Properties>,
    any: Option<Vec<Properties>>,
}

impl Clue {
    pub fn new() -> Clue {
        // Don't allow both fields to be None
        unimplemented!();
    }
    pub fn matches(&self, card: &Card) -> bool {
        if let Some(not_disjoint) = &self.not_disjoint {
            for (clue_prop, card_prop) in not_disjoint.iter().zip(&card.props) {
                if !clue_prop.is_disjoint(card_prop) {
                    return true;
                }
            }
        }
        if let Some(any) = &self.any {
            for properties in any {
                if *properties == card.props {
                    return true;
                }
            }
        }
        return false;
    }
}
