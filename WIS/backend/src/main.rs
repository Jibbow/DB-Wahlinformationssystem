#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate hdbconnect;
extern crate rocket;
extern crate handlebars;

use dotenv::dotenv;
use hdbconnect::{ConnectParams, Connection, HdbResult};
use std::env;
use rocket::State;


const test_query : &str = include_str!("../queries/test.sql");


#[get("/")]
fn test(db_connection: State<Connection>) -> String {
    format!("Hello, year old named {}!", test_query)
}

fn main() {
    // get configuration for database connection from environment or .env file
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("Please provide the DATABASE_URL as environment variable");
    let db_port = env::var("DATABASE_PORT")
        .expect("Please provide the DATABASE_PORT as environment variable")
        .parse::<u16>().unwrap();
    let db_user = env::var("DATABASE_USER")
        .expect("Please provide the DATABASE_USER as environment variable");
    let db_password = env::var("DATABASE_PASSWORD")
        .expect("Please provide the DATABASE_PASSWORD as environment variable");

    // setup connection parameters for SAP HANA
    let connect_params = ConnectParams::builder()
        .hostname(db_url)
        .port(db_port)
        .dbuser(db_user)
        .password(db_password)
        .build()
        .unwrap();
    let mut connection = Connection::new(connect_params);

    // start webserver
    rocket::ignite().manage(connection).mount("/test", routes![test]).launch();
}
