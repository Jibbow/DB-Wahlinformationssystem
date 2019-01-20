extern crate serde;
extern crate serde_json;

use rocket::response::content;
use rocket::response::status;
use rocket::State;


// load sql queries during compile time
const UEBERHANGMANDATE: &str = include_str!("../../queries/partei-überhangmandate.sql");
const KNAPPSTE_SIEGER: &str = include_str!("../../queries/partei-top10.sql");
const KNAPPSTE_VERLIERER: &str = include_str!("../../queries/partei-top-10-knappste-verlierer.sql");


/// [Q5]
/// Gibt für einen Wahlkreis und eine Partei die Anzahl der Überhangmandate zurück.
#[get("/ueberhangmandate/<wahlkreis>/<partei>/<jahr>")]
pub fn ueberhangmandate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, wahlkreis: u32, partei: u32, jahr: u32)
 -> Result<content::Json<String>, status::NotFound<&'static str>> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        UEBERHANGMANDATE: u32,
        WAHLKREIS: String,
        PARTEI: String
    }

    let query = UEBERHANGMANDATE
        .replace("{{WAHLKREIS}}", &wahlkreis.to_string())
        .replace("{{PARTEI}}", &partei.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    if result.len() == 0 {
        Err(status::NotFound("Die Partei ist in diesem Jahr nicht in den Landtag eingezogen und hat somit keine Überhangsmandate erhalten."))
    } else {
        Ok(content::Json(serde_json::to_string(&result[0]).unwrap()))
    }
}

/// [Q6 Teil 1]
/// Gibt die Top-10 Liste der knappsten Sieger mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Sieger sind die gewählten Erstkandidaten, welche mit dem geringsten
/// Vorsprung gegenüber ihren Konkurrenten gewonnen haben.
#[get("/knappstesieger/<partei>/<jahr>")]
pub fn knappstesieger(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, partei: u32, jahr: u32) -> content::Json<String> {
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

    let query = KNAPPSTE_SIEGER
        .replace("{{PARTEI}}", &partei.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q6 Teil 2]
/// /// Gibt die Top-10 Liste der knappsten Verlierer mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Verlierer sind die Erstkandidaten, welche mit dem geringsten
/// Abstand gegenüber ihren Konkurrenten verloren haben.
#[get("/knappsteverlierer/<partei>/<jahr>")]
pub fn knappsteverlierer(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, partei: u32, jahr: u32) -> content::Json<String> {
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

    let query = KNAPPSTE_VERLIERER
        .replace("{{PARTEI}}", &partei.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    //let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
    //    .query(&query).unwrap().try_into().unwrap();
    content::Json("not yet implemented".to_string()) // TODO
}

