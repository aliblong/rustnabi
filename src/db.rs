extern crate postgres;
use util::get_env_var;

pub fn init() -> postgres::Result<postgres::Connection> {
    let host = get_env_var("DB_HOST", "localhost".to_string());
    let port = get_env_var("DB_PORT", 5432.to_string());
    let user = get_env_var("DB_USER", "hanabi".to_string());
    let pw = get_env_var("DB_PW", "hanabi".to_string());
    let db_name = get_env_var("DB_NAME", "hanabi".to_string());
    let dsn = format!("postgresql://{}:{}@{}:{}/{}", user, pw, host, port, db_name);
    info!("{}", dsn);
    postgres::Connection::connect(dsn, postgres::TlsMode::None)
}
