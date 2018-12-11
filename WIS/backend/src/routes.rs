extern crate serde;
extern crate serde_json;
extern crate handlebars;

use super::get_db_connection;
use rocket::response::content;
use self::handlebars::{Handlebars};

// load sql queries during compile time
const SITZVERTEILUNG_QUERY: &str = include_str!("../queries/wahl-sitzverteilung.sql");
const LANDTAGSMITGLIEDER_QUERY: &str = include_str!("../queries/wahl-landtagsmitglieder.sql");
const WAHLKREIS_UEBERHANGMANDATE_QUERY: &str = include_str!("../queries/wahlkreis-überhangmandate.sql");
const STIMMKREIS_DIREKTKANDIDATENGEWINNER_QUERY: &str = include_str!("../queries/stimmkreis-direktkandidatengewinner.sql");
const STIMMKREIS_PARTEIERGEBNIS_QUERY: &str = include_str!("../queries/stimmkreis-parteiergebnis.sql");
const STIMMKREIS_PARTEIERGEBNIS_DIFFERENZ_QUERY: &str = include_str!("../queries/stimmkreis-parteiergebnis-differenz.sql");
const STIMMKREIS_SIEGERPARTEI_ERSTSTIMMER_QUERY: &str = include_str!("../queries/stimmkreis-siegerpartei-erststimmen.sql");
const STIMMKREIS_SIEGERPARTEI_ZWEITSTIMME_QUERY: &str = include_str!("../queries/stimmkreis-siegerpartei-zweitstimmen.sql");
const KNAPPSTE_SIEGER_QUERY: &str = include_str!("../queries/top-10.sql");
const KNAPPSTE_VERLIERER_QUERY: &str = include_str!("../queries/wahl-top-10-knappste-verlierer.sql");
const PARTEIEN_QUERY: &str = include_str!("../queries/wahl-parteien.sql");




/// [Q1]
/// Gibt die Sitzverteilung aller Parteien im Landtag zurück.
#[get("/sitzverteilung/<jahr>")]
pub fn sitzverteilung(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { PARTEI: String, SITZE: u32 }

    let result: Vec<Result> = get_db_connection().query(SITZVERTEILUNG_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q2]
/// Gibt eine Liste aller gewählten Landtagsmitglieder zurück.
#[get("/landtagsmitglieder/<jahr>")]
pub fn landtagsmitglieder(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { VORNAME: String, NACHNAME: String, PARTEI: String }

    let result: Vec<Result> = get_db_connection().query(LANDTAGSMITGLIEDER_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q3.1]
/// Gibt die Wahlbeteiligung für einen Stimmkreis zurück.
#[get("/wahlbeteiligung/<stimmkreis>/<jahr>")]
pub fn wahlbeteiligung(stimmkreis: u32, jahr: u32) -> content::Json<String> {
    content::Json("not yet implemented".to_string())
}


/// [Q3.2]
/// Gibt den gewählten Direktkandidaten für einen Stimmkreis zurück.
#[get("/direktkandidatengewinner/<stimmkreis>/<jahr>")]
pub fn direktkandidatengewinner(stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { ID: u32, VORNAME: String, NACHNAME: String, PARTEI: String }

    let reg = Handlebars::new();
    let query = reg.render_template(STIMMKREIS_DIREKTKANDIDATENGEWINNER_QUERY, &json!(
        {
            "JAHR": jahr,
            "STIMMKREIS": stimmkreis
        })).expect("Could not template query :(");

    let result: Vec<Result> = get_db_connection().query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q3.3]
/// Gibt die prozentuale und absolute Anzahl an Stimmen für jede Partei in einem Stimmkreis zurück.
#[get("/parteiergebnis/<stimmkreis>/<jahr>")]
pub fn parteiergebnis(stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { PARTEI: String, STIMMENABSOLUT: u32, STIMMENRELATIV: f32 }

    let reg = Handlebars::new();
    let query = reg.render_template(STIMMKREIS_PARTEIERGEBNIS_QUERY, &json!(
        {
            "JAHR": jahr,
            "STIMMKREIS": stimmkreis
        })).expect("Could not template query :(");

    let result: Vec<Result> = get_db_connection().query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q3.4]
/// Gibt die prozentuale und absolute Änderung an Stimmen für jede Partei in einem Stimmkreis zurück.
/// Die Änderung bezieht sich von 2013 auf 2018.
#[get("/parteiergebnisdifferenz/<stimmkreis>")]
pub fn parteiergebnisdifferenz(stimmkreis: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { PARTEI: String, DIFF_ABSOLUT: i32, DIFF_PROZENT: f32 }

    let reg = Handlebars::new();
    let query = reg.render_template(STIMMKREIS_PARTEIERGEBNIS_DIFFERENZ_QUERY, &json!(
        {
            "STIMMKREIS": stimmkreis
        })).expect("Could not template query :(");

    let result: Vec<Result> = get_db_connection().query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q4 Teil 1]
/// Gibt die Siegerparteien über die Erststimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/erststimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteierststimmen(stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result {  }

    let result: Vec<Result> = get_db_connection().query(STIMMKREIS_SIEGERPARTEI_ERSTSTIMMER_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q4 Teil 2]
/// Gibt die Siegerparteien über die Zweitstimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/zweitstimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteizweitstimmen(stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result {  }

    let result: Vec<Result> = get_db_connection().query(STIMMKREIS_SIEGERPARTEI_ZWEITSTIMME_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q5]
/// Gibt für jeden Wahlkreis die Überhangmandate pro Partei zurück.
#[get("/ueberhangmandate/<jahr>")]
pub fn ueberhangmandate(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { WAHLKREISID: String, PARTEI: String, UEBERHANGMANDATE: u32, WAHLKREIS: String }

    let result: Vec<Result> = get_db_connection().query(WAHLKREIS_UEBERHANGMANDATE_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q6 Teil 1]
/// Gibt die Top-10 Liste der knappsten Sieger mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Sieger sind die gewählten Erstkandidaten, welche mit dem geringsten
/// Vorsprung gegenüber ihren Konkurrenten gewonnen haben.
#[get("/knappstesieger/<jahr>")]
pub fn knappstesieger(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { ID: u32, JAHR: u32, DIFF: u32, VKANDIDAT: u32, PARTEI: u32, POS: u32, NACHNAME: String, VORNAME: String, ABKUERZUNG: String }

    let result: Vec<Result> = get_db_connection().query(KNAPPSTE_SIEGER_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// [Q6 Teil 2]
/// /// Gibt die Top-10 Liste der knappsten Verlierer mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Verlierer sind die Erstkandidaten, welche mit dem geringsten
/// Abstand gegenüber ihren Konkurrenten verloren haben.
#[get("/knappsteverlierer/<jahr>")]
pub fn knappsteverlierer(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result {  }

    let result: Vec<Result> = get_db_connection().query(KNAPPSTE_VERLIERER_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


/// Gibt einer Liste aller Parteien bei der Landtagswahl zurück.
#[get("/parteien")]
pub fn parteien() -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { ID: u32, NAME: String, ABKUERZUNG: String, FARBE: Option<String> } //FIXME: one option !!!!!!!!

    let result: Vec<Result> = get_db_connection().query(PARTEIEN_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


// MOCK!!!!!!!
#[get("/stimmverteilunggesamt/<jahr>")]
pub fn stimmverteilunggesamt(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { PARTEI: String, PROZENT: f32 }

    let result: Vec<Result> = vec![
        Result {
            PARTEI: "CSU".to_string(),
            PROZENT: 37.2
        },
        Result {
            PARTEI: "GRÜNE".to_string(),
            PROZENT: 17.6
        },
        Result {
            PARTEI: "FW".to_string(),
            PROZENT: 11.6
        },
        Result {
            PARTEI: "AfD".to_string(),
            PROZENT: 10.2
        },
        Result {
            PARTEI: "SPD".to_string(),
            PROZENT: 9.7
        },
        Result {
            PARTEI: "FDP".to_string(),
            PROZENT: 5.1
        },
        Result {
            PARTEI: "Linke".to_string(),
            PROZENT: 3.2
        },
        Result {
            PARTEI: "Sonstige".to_string(),
            PROZENT: 5.4
        }
    ];
    content::Json(serde_json::to_string(&result).unwrap())
}
