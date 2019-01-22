extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;


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

    let query = SITZVERTEILUNG
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query)?.try_into()?;
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

    let query = LANDTAGSMITGLIEDER
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query)?.try_into()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}



/// Gibt die prozentuale Verteilung aller Stimmen im Freistaat Bayern auf die Parteien zurück.
/// Gleiche Route wie für einen einzelnen Stimmkreis, aber das Argument für den Stimmkreis wird weggelassen.
#[get("/stimmverteilung/<jahr>")]
pub fn stimmverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32)
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

    let query = STIMMVERTEILUNG_GESAMT
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query)?.try_into()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}
