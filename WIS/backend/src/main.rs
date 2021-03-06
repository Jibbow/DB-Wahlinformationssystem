#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate hdbconnect;
extern crate r2d2;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod cors;
mod routes;

const BUILD_VERSION: Option<&'static str> = option_env!("BUILD_VERSION");


fn main() {
    // get configuration for database connection from environment or .env file
    dotenv::dotenv().ok();

    // set rocket port to $PORT if given and leave everything else untouched
    let mut rocket_config = rocket::config::Config::active().unwrap();
    if let Some(port) = std::env::var("PORT").map(|p| p.parse::<u16>().expect("Unable to parse $PORT")).ok() {
        rocket_config.set_port(port);
    }

    // start webserver
    rocket::custom(rocket_config).attach(cors::CORS())
        .manage(create_connection_pool())
        .mount("/api/hello", routes![hello])
        .mount(
            "/api",
            routes![
                routes::bayern::sitzverteilung,
                routes::bayern::landtagsmitglieder,
                routes::stimmkreis::wahlbeteiligung,
                routes::stimmkreis::direktkandidatengewinner,
                routes::stimmkreis::stimmverteilung,
                routes::stimmkreis::stimmverteilungdifferenz,
                routes::stimmkreis::siegerparteierststimmen,
                routes::stimmkreis::siegerparteizweitstimmen,
                routes::other::ueberhangmandate,
                routes::other::knappstesieger,
                routes::data::parteien,
                routes::data::stimmkreise,
                routes::bayern::stimmverteilung,
                routes::analysen::csu_sterberate,
                routes::analysen::fdp_gehalt,
                routes::stimmabgabe::abstimmen,
                routes::stimmabgabe::tokeninfo,
                routes::stimmabgabe::wahlzettel_erststimme,
                routes::stimmabgabe::wahlzettel_zweitstimme,
            ],
        )
        .launch();
}


/// This route may be used for latency/performance testing or for health checks
#[get("/")]
pub fn hello() -> &'static str {
    BUILD_VERSION.unwrap_or("Hi!")
}


/// Creates a new connection pool to SAP HANA based on the configuration
/// given by according environment variables.
fn create_connection_pool() -> r2d2::Pool<hdbconnect::ConnectionManager> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Please provide $DATABASE_URL env var");
    let database_port = std::env::var("DATABASE_PORT")
        .expect("Please provide $DATABASE_PORT env var")
        .parse::<u16>()
        .expect("Unable to parse $DATABASE_PORT");
    let database_user = std::env::var("DATABASE_USER")
        .expect("Please provide $DATABASE_USER env var");
    let database_password = std::env::var("DATABASE_PASSWORD")
        .expect("Please provide $DATABASE_PASSWORD env var");

    let db_connection_params = hdbconnect::ConnectParams::builder()
        .hostname(database_url)
        .port(database_port)
        .dbuser(database_user)
        .password(database_password)
        .build()
        .unwrap();

    r2d2::Pool::builder()
        .max_size(150)
        .max_lifetime(Some(std::time::Duration::from_secs(1)))
        .build_unchecked(hdbconnect::ConnectionManager::new(&db_connection_params))
}
