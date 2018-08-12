extern crate chrono;
extern crate dotenv;
extern crate pretty_env_logger;
extern crate ipnetwork;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

mod db;
mod util;

fn main() {
    pretty_env_logger::init();

    match dotenv::dotenv() {
        Err(e) => warn!("Error reading .env file: {}", e),
        _ => info!("Parsed .env file successfully"),
    }

    let db = db::connect();
    match db::add_user(&db, "testname", "testpw") {
        Ok(_) => info!("User successfully added"),
        Err(e) => info!("Error adding user: {}", e),
    }
    //http_server::init();
    //ws_server::init();
}
