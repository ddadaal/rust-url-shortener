// Generated by diesel_ext
#![allow(unused)]
#![allow(clippy::all)]

use super::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Jump {
    pub id: i32,
    pub from: String,
    pub to: String,
    pub create_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "stats"]
pub struct Stat {
    pub id: Option<i32>,
    pub ip: String,
    pub jump_from: String,
    pub url: String,
    pub time: NaiveDateTime,
}