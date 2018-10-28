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

use std::sync::mpsc;

struct Game {
    players: Vec<Player>,
    options: Options,
}

impl Game {
    pub fn new(players: Vec<Player>, options: Options) {
        Game { players, options }
    }

    pub fn run(&mut self) {
        let (game_tx, game_rx) = mpsc::channel();
        let (clock_tx, clock_rx) = mpsc::channel();
        chess_clock = ChessClock::new(
            options.n_players,
            options.base_time,
            options.time_per_turn
        );
        if options.timed {
            tokio::spawn(chess_clock.start(game_tx, clock_rx));
            let msg = self.start_message();
            loop {
                for active_player in players.iter().cycle() {
                    players.iter().map(|player| player.notify(msg));
                }
            }
        }
    }
>>>>>>> Stashed changes
}
