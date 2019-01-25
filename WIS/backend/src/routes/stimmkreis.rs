extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;
use rocket::http::Status;
use rocket::response::status::*;
use hdbconnect::HdbValue;


const WAHLBETEILIGUNG: &str = include_str!("../../queries/stimmkreis/wahlbeteiligung.sql");
const WAHLBETEILIGUNG_AGG: &str = include_str!("../../queries/stimmkreis/voraggregiert/agg-wahlbeteiligung.sql");
const DIREKTKANDIDATENGEWINNER: &str = include_str!("../../queries/stimmkreis/direktkandidatengewinner.sql");
const DIREKTKANDIDATENGEWINNER_AGG: &str = include_str!("../../queries/stimmkreis/voraggregiert/agg-direktkandidatengewinner.sql");
const STIMMVERTEILUNG: &str = include_str!("../../queries/stimmkreis/stimmverteilung.sql");
const STIMMVERTEILUNG_AGG: &str = include_str!("../../queries/stimmkreis/voraggregiert/agg-stimmverteilung.sql");
const STIMMVERTEILUNG_DIFF: &str = include_str!("../../queries/stimmkreis/stimmverteilung-diff.sql");
const STIMMVERTEILUNG_DIFF_AGG: &str = include_str!("../../queries/stimmkreis/voraggregiert/agg-stimmverteilung-diff.sql");
const SIEGERPARTEI_ERSTSTIMME: &str = include_str!("../../queries/stimmkreis/siegerpartei-erststimme.sql");
const SIEGERPARTEI_ZWEITSTIMME: &str = include_str!("../../queries/stimmkreis/siegerpartei-zweitstimme.sql");


/// [Q3.1]
/// Gibt die Wahlbeteiligung für einen Stimmkreis zurück.
#[get("/wahlbeteiligung/<stimmkreis>/<jahr>?<compute_on_aggreagted_data>")]
pub fn wahlbeteiligung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: i32, jahr: i32, compute_on_aggreagted_data: Option<bool>)
 -> Result<content::Json<String>, Custom<String>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        WAHLBETEILIGUNG: f32,
    }

    let query = match compute_on_aggreagted_data {
        Some(true) => WAHLBETEILIGUNG_AGG,
        _ => WAHLBETEILIGUNG
    };
    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        query, 
        vec![HdbValue::INT(stimmkreis), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}

/// [Q3.2]
/// Gibt den gewählten Direktkandidaten für einen Stimmkreis zurück.
#[get("/direktkandidatengewinner/<stimmkreis>/<jahr>?<compute_on_aggreagted_data>")]
pub fn direktkandidatengewinner(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: i32, jahr: i32, compute_on_aggreagted_data: Option<bool>)
 -> Result<content::Json<String>, Custom<String>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        VORNAME: String,
        NACHNAME: String,
        PARTEI: String,
    }

    let query = match compute_on_aggreagted_data {
        Some(true) => DIREKTKANDIDATENGEWINNER_AGG,
        _ => DIREKTKANDIDATENGEWINNER
    };
    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        query, 
        vec![HdbValue::INT(stimmkreis), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}

/// [Q3.3]
/// Gibt die prozentuale und absolute Anzahl an Stimmen für jede Partei in einem Stimmkreis zurück.
#[get("/stimmverteilung/<stimmkreis>/<jahr>?<compute_on_aggreagted_data>")]
pub fn stimmverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32, compute_on_aggreagted_data: Option<bool>)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        GESAMTSTIMMEN: u32,
        PROZENT: f32,
    }

    let query = match compute_on_aggreagted_data {
        Some(true) => STIMMVERTEILUNG_AGG,
        _ => STIMMVERTEILUNG
    }.replace("{{STIMMKREIS}}", &stimmkreis.to_string())
     .replace("{{JAHR}}", &jahr.to_string());
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}

/// [Q3.4]
/// Gibt die prozentuale und absolute Änderung an Stimmen für jede Partei in einem Stimmkreis zurück.
/// Die Änderung bezieht sich von 2013 auf 2018.
#[get("/stimmverteilungdifferenz/<stimmkreis>?<compute_on_aggreagted_data>")]
pub fn stimmverteilungdifferenz(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, compute_on_aggreagted_data: Option<bool>)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        DIFF_GESAMTSTIMMEN: i32,
        DIFF_PROZENT: f32,
    }

    let query = match compute_on_aggreagted_data {
        Some(true) => STIMMVERTEILUNG_DIFF_AGG,
        _ => STIMMVERTEILUNG_DIFF
    }.replace("{{STIMMKREIS}}", &stimmkreis.to_string());
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}

/// [Q4 Teil 1]
/// Gibt die Siegerpartei über die Erststimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/erststimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteierststimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        ANZAHLERSTSTIMMEN: u32,
    }

    let query = SIEGERPARTEI_ERSTSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result[0]).unwrap()))
}

/// [Q4 Teil 2]
/// Gibt die Siegerpartei über die Zweitstimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/zweitstimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteizweitstimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_FARBE: String,
        ANZAHLZWEITSTIMMEN: u32,
    }

    let query = SIEGERPARTEI_ZWEITSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result[0]).unwrap()))
}
