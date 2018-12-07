#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate hdbconnect;
extern crate rocket;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;
use hdbconnect::{ConnectParams, Connection};
use std::env;

mod routes;


lazy_static! {
    static ref DB_CONNECTION_PARAMS: ConnectParams = ConnectParams::builder()
        .hostname(env::var("DATABASE_URL").unwrap())
        .port(env::var("DATABASE_PORT").unwrap().parse::<u16>().unwrap())
        .dbuser(env::var("DATABASE_USER").unwrap())
        .password(env::var("DATABASE_PASSWORD").unwrap())
        .build()
        .unwrap();
}

fn main() {
    // get configuration for database connection from environment or .env file
    dotenv().ok();

    // test connection to SAP HANA
    Connection::new(DB_CONNECTION_PARAMS.clone())
        .expect("Could not establish connection to SAP HANA");

    // start webserver
    rocket::ignite().mount("/", routes![routes::test]).launch();
}


pub fn get_db_connection() -> Connection {
    // setup connection parameters for SAP HANA
    Connection::new(DB_CONNECTION_PARAMS.clone())
        .expect("Could not establish connection to SAP HANA")
}
