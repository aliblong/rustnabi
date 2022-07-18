mod card;
mod property;
mod zone;
mod clue;

use card::Card;
use zone::Zone;
use property::{{PropertyIndex, Properties}};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameSpecError {
    #[error("Invalid property value: {val}. Value must be an integer on [0, 2^7).")]
    InvalidClueIdx { val: i8 },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
