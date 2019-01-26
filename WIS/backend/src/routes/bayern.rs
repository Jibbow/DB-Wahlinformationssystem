extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;
use rocket::http::Status;
use rocket::response::status::*;
use hdbconnect::HdbValue;

const SITZVERTEILUNG: &str = include_str!("../../queries/bayern/sitzverteilung.sql");
const LANDTAGSMITGLIEDER: &str = include_str!("../../queries/bayern/landtagsmitglieder.sql");
const STIMMVERTEILUNG_GESAMT: &str = include_str!("../../queries/bayern/stimmverteilung.sql");


/// [Q1]
/// Gibt die Sitzverteilung aller Parteien im Landtag zurück.
#[get("/sitzverteilung/<jahr>")]
pub fn sitzverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        SITZE: u32,
    }

    let query = SITZVERTEILUNG;
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}

/// [Q2]
/// Gibt eine Liste aller gewählten Landtagsmitglieder zurück.
#[get("/landtagsmitglieder/<jahr>")]
pub fn landtagsmitglieder(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        VORNAME: String,
        NACHNAME: String,
        PARTEI: String,
    }

    let query = LANDTAGSMITGLIEDER;
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}



/// Gibt die prozentuale Verteilung aller Stimmen im Freistaat Bayern auf die Parteien zurück.
/// Gleiche Route wie für einen einzelnen Stimmkreis, aber das Argument für den Stimmkreis wird weggelassen.
#[get("/stimmverteilung/<jahr>")]
pub fn stimmverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: i32)
 -> Result<content::Json<String>, Custom<String>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        GESAMTSTIMMEN: u32,
        PROZENT: f32,
    }

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        STIMMVERTEILUNG_GESAMT, 
        vec![HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}
