extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod db;

fn main() {
    pretty_env_logger::init();

    match dotenv::dotenv() {
        Err(dotenv::Error::EnvVar(_)) => warn!("Weird error when reading .env file."),
        Err(dotenv::Error::Io(_)) => warn!("I/O error when reading .env file. File likely not found."),
        Err(dotenv::Error::LineParse(entry)) => warn!("Malformed entry in .env file: '{}'.", entry),
        _ => info!("Read .env file from {}", "test"),
    }

    db::init();
    //http_server::init();
    //ws_server::init();
}
