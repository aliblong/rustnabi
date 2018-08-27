use serde_yaml;
use db::models::{Suit, Index};

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub suits: Vec<Suit>,
    pub addl_colors: Option<Index>,
}

impl Variant {
    pub fn new(yaml: &str) -> Variant {
        serde_yaml::from_str(yaml).expect("Bad yaml")
    }
    pub fn suit(&self, i: Index) -> Result<&Suit, ()> {
        match self.suits.get(i as usize) {
            Some(suit) => Ok(suit),
            None => Err(()),
        }
    }
    pub fn suits(&self, i: Index) -> Vec<Index> {
        self.suits.iter().enumerate()
            .filter(|(j, suit)| suit.colors.contains(&i))
            .map(|(j, _)| j as Index)
            .collect()
    }
    // It's possible to determine the number of colors from the minimal representation of a variant
    // used in this model, so this number isn't stored in the data structure.
    pub fn n_colors(&self) -> Index {
        // I don't think there's a functional way to do this just from std.
        // `map_by_key` would return the suit, on which `max` could then be called a second time, but
        // that's an extra function call.
        // `flatten` doesn't work because the suit contains more data than just its colors
        let mut nominal_colors: usize = 0;
        for suit in self.suits.iter() {
            let max_color = match suit.colors.iter().max() {
                Some(&n) => n,
                None => 0,
            };
            if nominal_colors < max_color {
                nominal_colors = max_color;
            }
        }
        let addl_colors = match self.addl_colors {
            Some(n) => n,
            None => 0,
        };
        nominal_colors + addl_colors
    }
    // The variant is modeled as a map of {suit -> colors, ... }.
    // The inverse map { color -> suits } is useful for resolving clues.
    pub fn color_suit_map(&self) -> Vec<Vec<Index>> {
        let mut res: Vec<Vec<Index>> = Vec::with_capacity(2);
        for color in (0 .. self.n_colors()) {
            res.push(self.suits(color));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use test::*;
    use super::Variant;

    #[test]
    fn normal_variant() {
        let variant = Variant::new(NORMAL_VARIANT);
        assert_eq!(variant.suits(2)[0], 2);
    }

    #[test]
    fn acidtrip_variant() {
        let variant = Variant::new(ACID_TRIP_VARIANT);
        assert_eq!(variant.suits(2).len(), 0);
    }

    #[test]
    fn wildcrazy_variant() {
        let variant = Variant::new(WILD_CRAZY_VARIANT);
        assert_eq!(variant.suits(2).as_slice(), [1, 2, 4]);
        assert_eq!(variant.suits(3).as_slice(), [4, 5]);
    }
}
