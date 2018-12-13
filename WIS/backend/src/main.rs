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
extern crate serde_json;

mod cors;
mod routes;

fn main() {
    // get configuration for database connection from environment or .env file
    dotenv::dotenv().ok();

    // start webserver
    let config = rocket::config::Config::build(rocket::config::Environment::Production)
        .address("0.0.0.0")
        .port(8000)
        .unwrap();
    let app = rocket::custom(config);
    app.attach(cors::CORS())
        .manage(create_connection_pool())
        .mount(
            "/",
            routes![
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
            ],
        )
        .launch();
}

/// Creates a new connection pool to SAP HANA based on the configuration
/// given by according environment variables.
fn create_connection_pool() -> r2d2::Pool<hdbconnect::ConnectionManager> {
    let database_url = std::env::var("DATABASE_URL").expect("Please provide DATABASE_URL env var");
    let database_port = std::env::var("DATABASE_PORT")
        .expect("Please provide DATABASE_PORT env var")
        .parse::<u16>()
        .expect("Unable to parse DATABASE_PORT");
    let database_user =
        std::env::var("DATABASE_USER").expect("Please provide DATABASE_USER env var");
    let database_password =
        std::env::var("DATABASE_PASSWORD").expect("Please provide DATABASE_PASSWORD env var");

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
