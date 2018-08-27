use ipnetwork::IpNetwork;
use super::schema::users;
use chrono::NaiveDateTime as DT;
use serde_json;

pub type Index = usize;

#[derive(DbEnum, Debug)]
pub enum Endcon {
    //in progress, normal, strikeout, timeout, killed
    Inp,
    Norm,
    Strike,
    Time,
    Kill,
}

//#[derive(DbEnum, Debug)]
//pub enum Privilege {
//    Restart,
//    Ban,
//}

#[derive(DbEnum, Debug)]
pub enum Variant {
    Normal,
    Orange,
    Black,
    Rainbow,
    Dual,
    DualRainbow,
    WhiteRainbow,
    WildCrazy,
    Ambiguous,
    RedBlue,
    AcidTrip,
    DarkRainbow,
    DarkRainbowBlack,
}

#[derive(Queryable)]
pub struct User {
    id: i32,
    name: String,
    pw: Vec<u8>,
    salt: Vec<u8>,
    restart_privilege: bool,
    ban_privilege: bool,
    ip: Vec<IpNetwork>,
    datetime_last_login: DT,
    datetime_created: DT,
}

#[derive(Queryable)]
pub struct Table {
    id: i32,
    name: String,
    players: Vec<i32>,
    owner: i32,
    variant: Variant,
    timed: bool,
    seed: String,
    score: i16,
    endcon: Endcon,
    action: serde_json::Value,
    datetime_created: DT,
    datetime_started: DT,
    datetime_finished: DT,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct Suit {
    pub colors: Vec<Index>,
    pub dist: Vec<Index>,
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pw: &'a [u8],
    pub salt: &'a [u8],
    pub ip: &'a [IpNetwork],
}
