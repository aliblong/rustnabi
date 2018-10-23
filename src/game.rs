mod variant;
mod card;
mod deck;
mod hand;
mod player;
mod board;
mod play_area;
mod discard_area;
mod clue;

pub use self::variant::*;
pub use self::card::*;
pub use self::deck::*;
pub use self::hand::*;
pub use self::player::*;
pub use self::board::*;
pub use self::play_area::*;
pub use self::discard_area::*;
pub use self::clue::*;
pub use db::models::Index;

struct Game {
    players: Vec<Player>,
    active_player: std::iter::Iterator,
}
