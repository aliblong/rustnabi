use serde_yaml;

use super::Index;

#[derive(Debug, Deserialize)]
pub struct Suit {
    colors: Vec<i8>,
    dist: Vec<i8>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    suits: Vec<Suit>,
    addl_colors: Option<i8>,
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
    fn suits(&self, i: Index) -> Vec<Index> {
        self.suits.iter().enumerate()
            .filter(|(j, suit)| suit.colors.contains(&i))
            .map(|(j, _)| j as Index)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;
    #[test]
    fn normal_variant() {
        let yaml =
"---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors:
      - 0
  - dist: *def_dist
    colors:
      - 1
  - dist: *def_dist
    colors:
      - 2
  - dist: *def_dist
    colors:
      - 3
  - dist: *def_dist
    colors:
      - 4
";
        let variant = Variant::new(yaml);
        assert_eq!(variant.suits(2)[0], 2);
    }

    #[test]
    fn acidtrip_variant() {
        let yaml =
"---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors: []
";
        let variant = Variant::new(yaml);
        assert_eq!(variant.suits(2).len(), 0);
    }

    #[test]
    fn wildcrazy_variant() {
        let yaml =
"---
default_dist: &def_dist
  - 3
  - 2
  - 2
  - 2
  - 1

suits:
  - dist: *def_dist
    colors:
      - 0
      - 1
  - dist: *def_dist
    colors:
      - 0
      - 2
  - dist: *def_dist
    colors:
      - 1
      - 2
  - dist: *def_dist
    colors: []
  - dist: *def_dist
    colors:
      - 0
      - 1
      - 2
      - 3
  - dist:
      - 1
      - 1
      - 1
      - 1
      - 1
    colors:
      - 3
";
        let variant = Variant::new(yaml);
        assert_eq!(variant.suits(2).as_slice(), [1, 2, 4]);
        assert_eq!(variant.suits(3).as_slice(), [4, 5]);
    }
}
