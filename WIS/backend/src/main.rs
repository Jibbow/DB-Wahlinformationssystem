#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate hdbconnect;
extern crate r2d2;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use std::env;
use dotenv::dotenv;
use rocket::config::{Config, Environment};

mod routes;
mod cors;



lazy_static! {
    pub static ref CONNECTION_POOL: r2d2::Pool<hdbconnect::ConnectionManager> = create_connection_pool();
    static ref BACKEND_PORT: Option<u16> = env::var("BACKEND_PORT")
        .map(|s| {s.parse::<u16>().expect("Unable to parse BACKEND_PORT")}).ok();
}



fn main() {
    // get configuration for database connection from environment or .env file
    dotenv().ok();


    // start webserver
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(BACKEND_PORT.unwrap_or(8000))
        .unwrap();
    let app = rocket::custom(config);
    app.attach(cors::CORS()).mount("/", routes![
        routes::sitzverteilung,
        routes::landtagsmitglieder,
        routes::wahlbeteiligung,
        routes::direktkandidatengewinner,
        routes::parteiergebnis,
        routes::parteiergebnisdifferenz,
        routes::siegerparteierststimmen,
        routes::siegerparteizweitstimmen,
        routes::ueberhangmandate,
        routes::knappstesieger,
        routes::knappsteverlierer,
        routes::parteien,
        routes::stimmverteilunggesamt,
        routes::analysen_csu_sterberate,
        routes::analysen_fdp_einkommen,
        ]).launch();
}



fn create_connection_pool() -> r2d2::Pool<hdbconnect::ConnectionManager> {
    let database_url = env::var("DATABASE_URL")
        .expect("Please provide DATABASE_URL env var");
    let database_port = env::var("DATABASE_PORT")
        .expect("Please provide DATABASE_PORT env var")
        .parse::<u16>().expect("Unable to parse DATABASE_PORT");
    let database_user = env::var("DATABASE_USER")
        .expect("Please provide DATABASE_USER env var");
    let database_password = env::var("DATABASE_PASSWORD")
        .expect("Please provide DATABASE_PASSWORD env var");

    let db_connection_params = hdbconnect::ConnectParams::builder()
        .hostname(database_url)
        .port(database_port)
        .dbuser(database_user)
        .password(database_password)
        .build()
        .unwrap();

    r2d2::Pool::builder()
        .max_size(15)
        .build(hdbconnect::ConnectionManager::new(&db_connection_params))
        .unwrap()
}

