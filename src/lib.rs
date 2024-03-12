#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

mod db;
pub mod domain;