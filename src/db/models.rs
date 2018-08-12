use ipnetwork::IpNetwork;
use super::schema::users;
use chrono::NaiveDateTime as DT;

// define your enum
#[derive(DbEnum)]
#[derive(Debug)]
pub enum Endcon {
    //in progress, normal, strikeout, timeout, killed
    inp,
    norm,
    so,
    to,
    kill,
}

#[derive(Queryable)]
pub struct User {
    id: i32,
    name: String,
    pw: String,
    last_ip: Option<IpNetwork>,
    admin: i16,
    datetime_created: DT,
    datetime_last_login: DT,
}


#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub pw: &'a str,
}
