mod board;
mod card;
mod clue;
mod deck;
mod discard_area;
mod hand;
mod options;
mod play_area;
mod player;
mod variant;

pub use self::board::*;
pub use self::card::*;
pub use self::clue::*;
pub use self::deck::*;
pub use self::discard_area::*;
pub use self::hand::*;
pub use self::options::*;
pub use self::play_area::*;
pub use self::player::*;
pub use self::variant::*;
pub use db::models::Index;

use std::time::Duration;
use tokio::prelude::*;

use chess_clock::{BaseTime, ChessClock, ClockedFuture, TimePerTurn};
use futures::{future::Either, Future};

enum GameResult {
    Score(u8),
    Timeout,
    Strikeout,
}

enum Msg {
    Continue(String),
    End(GameResult),
}

struct Game {
    players: Vec<Player>,
    options: Options,
}

impl Game {
    pub fn start_msg(&self) -> String {
        unimplemented!();
    }

    pub fn new(players: Vec<Player>, options: Options) -> Game {
        Game { players, options }
    }

    fn run_timed(&mut self, chess_clock: ChessClock) -> GameResult {
        let mut send = self.start_msg();
        loop {
            for active_player in self.players.iter().cycle() {
                let chess_clock_handle = chess_clock.clone();
                self.players
                    .iter()
                    .map(move |player| player.recv(send.clone()));
                let wait_turn = chess_clock_handle
                    .bind(active_player.send())
                    .wait()
                    .unwrap();
                match wait_turn {
                    Some(recv) => match self.recv(recv) {
                        Ok(Msg::Continue(to_send)) => send = to_send,
                        Ok(Msg::End(result)) => return result,
                        Err(_) => unimplemented!(),
                    },
                    None => return GameResult::Timeout,
                }
            }
        }
    }

    fn run_untimed(&mut self) -> GameResult {
        let mut send = self.start_msg();
        loop {
            for active_player in self.players.iter().cycle() {
                self.players
                    .iter()
                    .map(move |player| player.recv(send.clone()));
                let try_recv = await!(async {
                    active_player.send().timeout(Duration::from_secs(1800));
                });
                match try_recv {
                    Ok(recv) => match self.recv(recv) {
                        Ok(Msg::Continue(to_send)) => send = to_send,
                        Ok(Msg::End(result)) => return result,
                        Err(_) => unimplemented!(),
                    },
                    Err(_) => GameResult::Timeout,
                }
            }
        }
    }

    pub fn run(&mut self) -> GameResult {
        match self.options.timed {
            Some(TimerConfig {
                base_time,
                time_per_turn,
            }) => {
                let chess_clock = ChessClock::new(
                    self.options.n_players as usize,
                    BaseTime(Duration::from_secs(base_time)),
                    TimePerTurn(Duration::from_secs(time_per_turn)),
                );
                self.run_timed(chess_clock)
            }
            None => {
                unimplemented!();
            }
        }
    }
    fn recv(&self, msg: &str) -> Result<Msg, ()> {
        unimplemented!();
    }
}
