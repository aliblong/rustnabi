extern crate chrono;
extern crate dotenv;
extern crate pretty_env_logger;
extern crate ipnetwork;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
extern crate ring;
#[macro_use] extern crate lazy_static;

mod db;
mod util;
//mod login;

use ring::rand::SystemRandom;

/// According to `ring` docs, one (threadsafe) instance of SystemRandom should be used for the
/// entire app
lazy_static! {
    pub static ref SYSRAND: SystemRandom = SystemRandom::new();
}

fn main() {
    pretty_env_logger::init();

    match dotenv::dotenv() {
        Err(e) => warn!("Error reading .env file: {}", e),
        _ => warn!("Parsed .env file successfully"),
    }

    let db = db::Db::connect();
    let name = "testname3";
    let pw = b"asdf";
    match db.authenticate_user(name, pw.to_vec()) {
        Err(_) => warn!("Invalid credentials for {}", name),
        _ => warn!("User {} logged in successfully", name),
    }
    //http_server::init();
    //ws_server::init();
}
