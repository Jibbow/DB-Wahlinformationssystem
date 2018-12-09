#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate hdbconnect;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::env;
use dotenv::dotenv;
use hdbconnect::{ConnectParams, Connection};
use rocket::config::{Config, Environment};

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
    static ref BACKEND_PORT: Option<u16> = env::var("BACKEND_PORT")
        .map(|s| {s.parse::<u16>().expect("Unable to parse BACKEND_PORT")}).ok();
}


fn main() {
    // get configuration for database connection from environment or .env file
    dotenv().ok();

    // test connection to SAP HANA
    Connection::new(DB_CONNECTION_PARAMS.clone())
        .expect("Could not establish connection to SAP HANA");

    // start webserver
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(BACKEND_PORT.unwrap_or(8000))
        .unwrap();
    let app = rocket::custom(config);
    app.mount("/", routes![routes::test]).launch();
}


pub fn get_db_connection() -> Connection {
    // setup connection parameters for SAP HANA
    Connection::new(DB_CONNECTION_PARAMS.clone())
        .expect("Could not establish connection to SAP HANA")
}
