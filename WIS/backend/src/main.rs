#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate hdbconnect;
extern crate rocket;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::env;
use dotenv::dotenv;
use hdbconnect::{ConnectParams, Connection};

mod routes;



lazy_static! {
    static ref DB_CONNECTION_PARAMS: ConnectParams = ConnectParams::builder()
        .hostname(env::var("DATABASE_URL")
            .expect("Please provide DATABASE_URL env var"))
        .port(env::var("DATABASE_PORT")
            .expect("Please provide DATABASE_PORT env var").parse::<u16>().expect("Unable to parse DATABASE_PORT"))
        .dbuser(env::var("DATABASE_USER")
            .expect("Please provide DATABASE_USER env var"))
        .password(env::var("DATABASE_PASSWORD")
            .expect("Please provide DATABASE_PASSWORD env var"))
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
