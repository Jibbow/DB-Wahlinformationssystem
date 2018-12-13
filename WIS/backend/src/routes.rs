extern crate handlebars;
extern crate serde;
extern crate serde_json;

use self::handlebars::Handlebars;
use rocket::response::content;
use rocket::State;


// load sql queries during compile time
const SITZVERTEILUNG: &str = include_str!("../queries/sitzverteilung.sql");
const LANDTAGSMITGLIEDER: &str = include_str!("../queries/landtagsmitglieder.sql");
const UEBERHANGMANDATE: &str = include_str!("../queries/wk-überhangmandate.sql");
const DIREKTKANDIDATENGEWINNER: &str = include_str!("../queries/sk-direktkandidatengewinner.sql");
const PARTEIERGEBNIS: &str = include_str!("../queries/sk-parteiergebnis.sql");
const PARTEIERGEBNIS_DIFF: &str = include_str!("../queries/sk-parteiergebnis-diff.sql");
const SIEGERPARTEI_ERSTSTIMME: &str = include_str!("../queries/sk-siegerpartei-erststimme.sql");
const SIEGERPARTEI_ZWEITSTIMME: &str = include_str!("../queries/sk-siegerpartei-zweitstimme.sql");
const KNAPPSTE_SIEGER: &str = include_str!("../queries/top10.sql");
const KNAPPSTE_VERLIERER: &str = include_str!("../queries/top-10-knappste-verlierer.sql");
const PARTEIEN: &str = include_str!("../queries/parteien.sql");
const ANALYSIS_CSU_AGE: &str = include_str!("../queries/analysis-csu-age.sql");
const ANALYSIS_FDP_INCOME: &str = include_str!("../queries/analysis-fdp-income.sql");





/// [Q1]
/// Gibt die Sitzverteilung aller Parteien im Landtag zurück.
#[get("/sitzverteilung/<jahr>")]
pub fn sitzverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        SITZE: u32,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(SITZVERTEILUNG).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q2]
/// Gibt eine Liste aller gewählten Landtagsmitglieder zurück.
#[get("/landtagsmitglieder/<jahr>")]
pub fn landtagsmitglieder(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        VORNAME: String,
        NACHNAME: String,
        PARTEI: String,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(LANDTAGSMITGLIEDER).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q3.1]
/// Gibt die Wahlbeteiligung für einen Stimmkreis zurück.
#[get("/wahlbeteiligung/<stimmkreis>/<jahr>")]
pub fn wahlbeteiligung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    content::Json("not yet implemented".to_string())
}

/// [Q3.2]
/// Gibt den gewählten Direktkandidaten für einen Stimmkreis zurück.
#[get("/direktkandidatengewinner/<stimmkreis>/<jahr>")]
pub fn direktkandidatengewinner(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        VORNAME: String,
        NACHNAME: String,
        PARTEI: String,
    }

    let reg = Handlebars::new();
    let query = reg.render_template(
        DIREKTKANDIDATENGEWINNER,
        &json!(
        {
            "JAHR": jahr,
            "STIMMKREIS": stimmkreis
        })).expect("Could not template query :(");

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q3.3]
/// Gibt die prozentuale und absolute Anzahl an Stimmen für jede Partei in einem Stimmkreis zurück.
#[get("/parteiergebnis/<stimmkreis>/<jahr>")]
pub fn parteiergebnis(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        STIMMENABSOLUT: u32,
        STIMMENRELATIV: f32,
    }

    let reg = Handlebars::new();
    let query = reg.render_template(
        PARTEIERGEBNIS,
        &json!(
        {
            "JAHR": jahr,
            "STIMMKREIS": stimmkreis
        })).expect("Could not template query :(");

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q3.4]
/// Gibt die prozentuale und absolute Änderung an Stimmen für jede Partei in einem Stimmkreis zurück.
/// Die Änderung bezieht sich von 2013 auf 2018.
#[get("/parteiergebnisdifferenz/<stimmkreis>")]
pub fn parteiergebnisdifferenz(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        DIFF_ABSOLUT: i32,
        DIFF_PROZENT: f32,
    }

    let reg = Handlebars::new();
    let query = reg.render_template(
        PARTEIERGEBNIS_DIFF,
        &json!(
        {
            "STIMMKREIS": stimmkreis 
        })).expect("Could not template query :(");

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q4 Teil 1]
/// Gibt die Siegerparteien über die Erststimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/erststimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteierststimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {}

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(SIEGERPARTEI_ERSTSTIMME).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q4 Teil 2]
/// Gibt die Siegerparteien über die Zweitstimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/zweitstimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteizweitstimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {}

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(SIEGERPARTEI_ZWEITSTIMME).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q5]
/// Gibt für jeden Wahlkreis die Überhangmandate pro Partei zurück.
#[get("/ueberhangmandate/<jahr>")]
pub fn ueberhangmandate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        WAHLKREISID: String,
        PARTEI: String,
        UEBERHANGMANDATE: u32,
        WAHLKREIS: String,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(UEBERHANGMANDATE).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q6 Teil 1]
/// Gibt die Top-10 Liste der knappsten Sieger mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Sieger sind die gewählten Erstkandidaten, welche mit dem geringsten
/// Vorsprung gegenüber ihren Konkurrenten gewonnen haben.
#[get("/knappstesieger/<jahr>")]
pub fn knappstesieger(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        JAHR: u32,
        DIFF: i32,
        VKANDIDAT: u32,
        PARTEI: u32,
        POS: u32,
        NACHNAME: String,
        VORNAME: String,
        ABKUERZUNG: String,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(KNAPPSTE_SIEGER).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q6 Teil 2]
/// /// Gibt die Top-10 Liste der knappsten Verlierer mit ihren Stimmkreisen aller Parteien dar.
/// Die knappsten Verlierer sind die Erstkandidaten, welche mit dem geringsten
/// Abstand gegenüber ihren Konkurrenten verloren haben.
#[get("/knappsteverlierer/<jahr>")]
pub fn knappsteverlierer(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {}

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(KNAPPSTE_VERLIERER).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Gibt einer Liste aller Parteien bei der Landtagswahl zurück.
#[get("/parteien")]
pub fn parteien(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        NAME: String,
        ABKUERZUNG: String,
        FARBE: Option<String>,
    } //FIXME: ohne option !!!!!!!!

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(PARTEIEN).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Vergleicht die Sterberate mit der Prozentualen Anzahl der CSU-Wähler
#[get("/analysen/csu-sterberate")]
pub fn analysen_csu_sterberate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PROZENT: f32,
        PARTEI: String,
        STERBERATE: f32,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(ANALYSIS_CSU_AGE).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Vergleicht das Durchschnittseinkommen mit der Prozentualen Anzahl der FDP-Wähler
#[get("/analysen/fdp-einkommen")]
pub fn analysen_fdp_einkommen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PROZENT: f32,
        PARTEI: String,
        EINKOMMEN: u32,
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(ANALYSIS_FDP_INCOME).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

// MOCK!!!!!!!
#[get("/stimmverteilunggesamt/<jahr>")]
pub fn stimmverteilunggesamt(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PROZENT: f32,
    }

    let result: Vec<QueryResult> = vec![
        QueryResult {
            PARTEI: "CSU".to_string(),
            PROZENT: 37.2,
        },
        QueryResult {
            PARTEI: "GRÜNE".to_string(),
            PROZENT: 17.6,
        },
        QueryResult {
            PARTEI: "FW".to_string(),
            PROZENT: 11.6,
        },
        QueryResult {
            PARTEI: "AfD".to_string(),
            PROZENT: 10.2,
        },
        QueryResult {
            PARTEI: "SPD".to_string(),
            PROZENT: 9.7,
        },
        QueryResult {
            PARTEI: "FDP".to_string(),
            PROZENT: 5.1,
        },
        QueryResult {
            PARTEI: "Linke".to_string(),
            PROZENT: 3.2,
        },
        QueryResult {
            PARTEI: "Sonstige".to_string(),
            PROZENT: 5.4,
        },
    ];
    content::Json(serde_json::to_string(&result).unwrap())
}
