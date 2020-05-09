pub mod database;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use database::connect_database::connect_database;
use database::models::*;

fn main() {
    use schema::account_holder::dsl::*;
    let connection = connect_database();
    let results = account_holder
        .filter(id.eq(1))
        .limit(1)
        .load::<AccountHolder>(&connection)
        .expect("Err");

    println!("Showing users {}", results.len());

    for usr in results {
        println!("name {}", usr.first_name)
    }
}
