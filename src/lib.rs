#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::pg::Pg;

type DB = Pg;

pub mod schema;
pub mod models;
pub mod kullanicilar;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let veritabani_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL bir değere sahip olmalıdır.");
    PgConnection::establish(&veritabani_url)
        .expect(&format!("{} veri tabanına bağlantı hatası oluştu!", veritabani_url))
}