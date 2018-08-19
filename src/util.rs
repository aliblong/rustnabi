/*
use chrono::{Utc, NaiveDateTime};

pub fn current_dt() -> NaiveDateTime {
    Utc::now().naive_utc()
}
use std::env::{var, VarError};

pub fn get_env_var(key: &str, default_val: String) -> String {
    match var(key) {
        Ok(val) => val,
        Err(VarError::NotPresent) => {
            warn!(
                "Missing entry {} in .env; defaulting to {}",
                key, default_val
            );
            default_val
        }
        Err(VarError::NotUnicode(_)) => {
            warn!(
                "Entry {} in .env is not valid Unicode; defaulting to {}",
                key, default_val
            );
            default_val
        }
    }
}
*/
