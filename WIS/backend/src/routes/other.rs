extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;
use rocket::http::Status;
use rocket::response::status::*;
use hdbconnect::HdbValue;


// load sql queries during compile time
const UEBERHANGMANDATE: &str = include_str!("../../queries/partei-überhangmandate.sql");
const KNAPPSTE_SIEGER: &str = include_str!("../../queries/partei-top10.sql");


/// [Q5]
/// Gibt für einen Wahlkreis und eine Partei die Anzahl der Überhangmandate zurück.
#[get("/ueberhangmandate/<wahlkreis>/<partei>/<jahr>")]
pub fn ueberhangmandate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, wahlkreis: i32, partei: i32, jahr: i32)
 -> Result<content::Json<String>, Custom<String>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        UEBERHANGMANDATE: u32,
        WAHLKREIS: String,
        PARTEI: String
    }

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        UEBERHANGMANDATE, 
        vec![HdbValue::INT(wahlkreis), HdbValue::INT(partei), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => if r.len() == 0 {
                    Err(Custom(Status::NotFound, "Die Partei ist in diesem Jahr nicht in den Landtag eingezogen und hat somit keine Überhangmandate erhalten.".to_string()))
                } else {
                    Ok(content::Json(serde_json::to_string(&r[0]).unwrap()))
                },
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}

/// [Q6 Teil 1]
/// Gibt die Top-10 Liste der knappsten Sieger mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Sieger sind die gewählten Erstkandidaten, welche mit dem geringsten
/// Vorsprung gegenüber ihren Konkurrenten gewonnen haben.
#[get("/knappstesieger/<partei>/<jahr>")]
pub fn knappstesieger(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, partei: i32, jahr: i32)
 -> Result<content::Json<String>, Custom<String>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        VORNAME: String,
        NACHNAME: String,
        PLATZIERUNG: u32,
        DIFFERENZ: i32,
        RIVALE: u32,
    }

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        KNAPPSTE_SIEGER, 
        vec![HdbValue::INT(partei), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}
