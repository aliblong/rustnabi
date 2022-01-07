# rustnabi

`rustnabi` is a webserver for multiplayer Hanabi, a card game of cooperative deduction. As the name implies, it is written in Rust. It is currently a WIP.

# Install guide for Debian-based Linux distros

## Rust, Cargo

```
$ curl https://sh.rustup.rs -sSf | sh
```

Use the `nightly` version. May need to run `rustup default nightly` at some point.

## PostgreSQL

```
sudo apt install postgresql postgresql-client
sudo -u postgres bash
psql
create user hanabi with password 'hanabi';
create database hanabi;
grant all privileges on database hanabi to hanabi;
```

The default entry in `.env` should point to your new database. Update it if you changed any of the parameters above.

```
$ cargo install diesel-cli
$ diesel migration run
$ ./refresh_sql.sh
```

# Useful commands

* `./refresh_sql.sh` to update SQL schema
* `cargo build` to compile
* `RUST_LOG=warn cargo run` to run, but it doesn't do much in this WIP state.
* `cargo test` to run tests
* `cargo doc` to compile docs that you can use offline
