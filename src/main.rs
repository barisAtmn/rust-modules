#![allow(dead_code)]
#![allow(unused_variables)]

mod auth;
mod database;

use database::Connection;
use crate::auth::User;

fn main() {
    let database = Connection::new(String::from("test"));
    let user = database.get_data(4);

    let control = User::age_control(user.unwrap());

    println!("User is older than 18: ({})", control);

}
