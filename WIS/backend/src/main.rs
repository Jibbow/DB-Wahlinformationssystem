#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate handlebars;
extern crate hdbconnect;
extern crate rocket;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use hdbconnect::{ConnectParams, Connection};
use std::env;

// load sql queries during compile time
const TEST_QUERY: &str = include_str!("../queries/test.sql");


fn main() {
    // start webserver
    rocket::ignite().mount("/", routes![test]).launch();
}


#[get("/test")]
fn test() -> String {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { SITZZAHL: usize, NAME: String, NR: usize }

    let result: Vec<Result> = get_db_connection().query(TEST_QUERY).unwrap().try_into().unwrap();
    serde_json::to_string(&result).unwrap()
}


fn get_db_connection() -> Connection {
    // get configuration for database connection from environment or .env file
    dotenv().ok();
    let db_url =
        env::var("DATABASE_URL").expect("Please provide the DATABASE_URL as environment variable");
    let db_port = env::var("DATABASE_PORT")
        .expect("Please provide the DATABASE_PORT as environment variable")
        .parse::<u16>()
        .unwrap();
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

    Connection::new(connect_params).expect("Could not establish connection to SAP HANA")
}
