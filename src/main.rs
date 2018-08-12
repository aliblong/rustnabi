extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod db;
mod util;

fn main() {
    pretty_env_logger::init();

    match dotenv::dotenv() {
        Err(e) => warn!("Error reading .env file: {}", e),
        _ => info!("Parsed .env file successfully"),
    }

    let db = match db::init() {
        Err(e) => {
            error!("DB connection error: {}", e);
            return;
        }
        Ok(connection) => connection,
    };
    //http_server::init();
    //ws_server::init();
}
