extern crate serde;
extern crate serde_json;
extern crate regex;

use rocket::response::content;
use rocket::State;
use rocket::response::status::*;


// load sql queries during compile time
const SITZVERTEILUNG: &str = include_str!("../queries/bayern-sitzverteilung.sql");
const LANDTAGSMITGLIEDER: &str = include_str!("../queries/bayern-landtagsmitglieder.sql");
const STIMMVERTEILUNG_GESAMT: &str = include_str!("../queries/bayern-stimmverteilung-gesamt.sql");
const UEBERHANGMANDATE: &str = include_str!("../queries/partei-überhangmandate.sql");
const KNAPPSTE_SIEGER: &str = include_str!("../queries/partei-top10.sql");
const KNAPPSTE_VERLIERER: &str = include_str!("../queries/partei-top-10-knappste-verlierer.sql");
const WAHLBETEILIGUNG: &str = include_str!("../queries/sk-wahlbeteiligung.sql");
const DIREKTKANDIDATENGEWINNER: &str = include_str!("../queries/sk-direktkandidatengewinner.sql");
const STIMMVERTEILUNG: &str = include_str!("../queries/sk-stimmverteilung.sql");
const STIMMVERTEILUNG_DIFF: &str = include_str!("../queries/sk-stimmverteilung-diff.sql");
const SIEGERPARTEI_ERSTSTIMME: &str = include_str!("../queries/sk-siegerpartei-erststimme.sql");
const SIEGERPARTEI_ZWEITSTIMME: &str = include_str!("../queries/sk-siegerpartei-zweitstimme.sql");
const ANALYSIS_CSU_AGE: &str = include_str!("../queries/analysis-csu-age.sql");
const ANALYSIS_FDP_INCOME: &str = include_str!("../queries/analysis-fdp-income.sql");
const PARTEIEN: &str = include_str!("../queries/tabelle-parteien.sql");
const STIMMKREISE: &str = include_str!("../queries/tabelle-stimmkreise.sql");
const STIMMZETTEL_ERSTSTIMME: &str = include_str!("../queries/wahl-stimmzettel-erststimme.sql");
const STIMMZETTEL_ZWEITSTIMME: &str = include_str!("../queries/wahl-stimmzettel-zweitstimme.sql");





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

    let query = SITZVERTEILUNG
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
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
        ID: u32,
        VORNAME: String,
        NACHNAME: String,
        PARTEI: String,
    }

    let query = LANDTAGSMITGLIEDER
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q3.1]
/// Gibt die Wahlbeteiligung für einen Stimmkreis zurück.
#[get("/wahlbeteiligung/<stimmkreis>/<jahr>")]
pub fn wahlbeteiligung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        WAHLBETEILIGUNG: f32,
    }

    let query = WAHLBETEILIGUNG
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result[0]).unwrap())
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

    let query = DIREKTKANDIDATENGEWINNER
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result[0]).unwrap())
}

/// [Q3.3]
/// Gibt die prozentuale und absolute Anzahl an Stimmen für jede Partei in einem Stimmkreis zurück.
#[get("/stimmverteilung/<stimmkreis>/<jahr>")]
pub fn stimmverteilung(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        GESAMTSTIMMEN: u32,
        PROZENT: f32,
    }

    let query = STIMMVERTEILUNG
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q3.4]
/// Gibt die prozentuale und absolute Änderung an Stimmen für jede Partei in einem Stimmkreis zurück.
/// Die Änderung bezieht sich von 2013 auf 2018.
#[get("/stimmverteilungdifferenz/<stimmkreis>")]
pub fn stimmverteilungdifferenz(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        DIFF_GESAMTSTIMMEN: i32,
        DIFF_PROZENT: f32,
    }

    let query = STIMMVERTEILUNG_DIFF
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// [Q4 Teil 1]
/// Gibt die Siegerpartei über die Erststimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/erststimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteierststimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        ANZAHLERSTSTIMMEN: u32,
    }

    let query = SIEGERPARTEI_ERSTSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result[0]).unwrap())
}

/// [Q4 Teil 2]
/// Gibt die Siegerpartei über die Zweitstimmen für einen Stimmkreis zurück.
#[get("/siegerpartei/zweitstimmen/<stimmkreis>/<jahr>")]
pub fn siegerparteizweitstimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        ANZAHLZWEITSTIMMEN: u32,
    }

    let query = SIEGERPARTEI_ZWEITSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result[0]).unwrap())
}

/// [Q5]
/// Gibt für einen Wahlkreis und eine Partei die Anzahl der Überhangmandate zurück.
#[get("/ueberhangmandate/<wahlkreis>/<partei>/<jahr>")]
pub fn ueberhangmandate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, wahlkreis: u32, partei: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        UEBERHANGMANDATE: u32,
    }

    let query = UEBERHANGMANDATE
        .replace("{{WAHLKREIS}}", &wahlkreis.to_string())
        .replace("{{PARTEI}}", &partei.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result[0]).unwrap())
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

/// Gibt eine Liste aller Parteien bei der Landtagswahl zurück.
#[get("/parteien")]
pub fn parteien(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        NAME: String,
        ABKUERZUNG: String,
        FARBE: String,
    }

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(PARTEIEN).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Gibt eine Liste aller Stimmkreise in Bayern zurück.
/// Vorsicht: die IDs der Stimmkreise ändern sich über die Jahre hinweg!
#[get("/stimmkreise/<jahr>")]
pub fn stimmkreise(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        NR: u32,
        NAME: String,
        STIMMBERECHTIGTE: u32,
        WAHLKREIS: String,
        WAHLKREISNR: u32,
    }

    let query = STIMMKREISE
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Gibt die prozentuale Verteilung aller Stimmen im Freistaat Bayern auf die Parteien zurück.
/// Gleiche Route wie für einen einzelnen Stimmkreis, aber das Argument für den Stimmkreis wird weggelassen.
#[get("/stimmverteilung/<jahr>")]
pub fn stimmverteilunggesamt(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        GESAMTSTIMMEN: u32,
        PROZENT: f32,
    }

    let query = STIMMVERTEILUNG_GESAMT
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

#[get("/wahlzettel/erststimme/<stimmkreis>/<jahr>")]
pub fn wahlzettel_erststimme(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_ABKUERZUNG: String,
        KANDIDAT_VORNAME: String,
        KANDIDAT_NACHNAME: String,
        LISTENPOSITION: String,
    }

    let query = STIMMZETTEL_ERSTSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

#[get("/wahlzettel/zweitstimme/<stimmkreis>/<jahr>")]
pub fn wahlzettel_zweitstimme(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: u32, jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PARTEI: String,
        PARTEI_ABKUERZUNG: String,
        KANDIDAT_VORNAME: String,
        KANDIDAT_NACHNAME: String,
        LISTENPOSITION: String,
    }

    let query = STIMMZETTEL_ZWEITSTIMME
        .replace("{{STIMMKREIS}}", &stimmkreis.to_string())
        .replace("{{JAHR}}", &jahr.to_string());

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

#[get("/tokeninfo/<token>")]
pub fn tokeninfo(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, token: String) -> Result<content::Json<String>, BadRequest<&'static str>> {
    // validate token
    let validator = regex::Regex::new(r"^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$").unwrap();
    if !validator.is_match(&token) {
        return Err(BadRequest(Some("Format des Tokens ist ungültig.")));
    }

    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        WAHLTOKEN: String,
	    JAHR: u32,
	    STIMMKREIS: u32,
	    ERSTSTIMMEABGEGEBEN: u32,
	    ZWEITSTIMMEABGEGEBEN: u32,
    }

    let query = "SELECT * FROM WIS.WAHLTOKEN WHERE WAHLTOKEN='{{TOKEN}}'"
        .replace("{{TOKEN}}", &token);

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(&query).unwrap().try_into().unwrap();
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
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
        STERBERATE: f64,
    }

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(ANALYSIS_CSU_AGE).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}

/// Vergleicht das Durchschnittsgehalt mit der Prozentualen Anzahl der FDP-Wähler
#[get("/analysen/fdp-gehalt")]
pub fn analysen_fdp_gehalt(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PROZENT: f32,
        PARTEI: String,
        GEHALT: u32,
    }

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(ANALYSIS_FDP_INCOME).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}



#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum Erststimme {
    kandidat(u32),
    enthaltung,
}
#[derive(Deserialize)]
#[allow(non_camel_case_types)]
pub enum Zweitstimme {
    partei(u32),
    kandidat(u32),
    enthaltung,
}
#[derive(Deserialize)]
pub struct Stimmabgabe {
    token: String,
    erststimme: Option<Erststimme>,
    zweitstimme: Option<Zweitstimme>,
}

/// Mit dieser Anfrage kann man eine Stimme abgeben, die dann im Wahlsystem gespeichert wird,
/// falls sie gültig ist und das Token (=Ausweisnummer) noch nicht für die jeweilige Stimme
/// eingesetzt wurde.
/// Dabei gibt es einen wesentlichen Unterschied zwischen "Enthaltung" und "Stimme (noch) nicht abgegeben".
/// Wir wolles es in unserem System möglich machen, dass man nicht sofort immer Erst- und Zweitstimme
/// abgeben muss. Deshalb kann man im JSON die Erst- und Zweitstimme jeweils auf "null" setzen. Das heißt
/// dann, dass die Stimme (noch) nicht abgegeben wurde und sie möglicherweise zu einem späteren
/// Zeitpunkt noch abgegeben wird. Wenn dagegen die Erst- oder Zweitstimme nicht auf "null" sind,
/// aber auf "enthaltung",dann wird die Stimme im System als gültig registriert, aber in der Datebank
/// wird einfach ein "null" Wert angelegt für den gewählten Kandidaten / die gewählte Partei.
/// ## Beispiel 1:
/// Erststimme ist eine Enthaltung, bei der Zweitstimme wurde die Partei mit der ID 5 gewählt.
/// ```
/// {
///  "token": "token-token-token-token",
///  "erststimme": "enthaltung",
///  "zweitstimme": {
///    "partei": 5
///  }
/// }
/// ```
/// ## Beispiel 2:
/// Bei der Erststimme wurde der Kandidat mit der ID 5 gewählt, die Zweitstimme wurde nicht abgegeben.
/// ```
/// {
///  "token": "token-token-token-token",
///  "erststimme": {
///    "kandidat": 5
///  },
///  "zweitstimme": null
/// }
/// ```
#[post("/abstimmen", data = "<stimme>")]
pub fn abstimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimme: rocket_contrib::json::Json<Stimmabgabe>) -> Result<&'static str, BadRequest<&'static str>> {
    // validate token
    let validator = regex::Regex::new(r"^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$").unwrap();
    if !validator.is_match(&stimme.token) {
        return Err(BadRequest(Some("Format des Tokens ist ungültig.")));
    }

    let mut db_connection = db.get().expect("failed to connect to DB");

    if let Some(ref erststimme) = stimme.erststimme {
        // falls eine Erststimme mitgeliefert wurde, dann wird in der Datenbank das Token markiert als "Erststimme abgegeben"
        let query = "UPDATE WIS.WAHLTOKEN SET ERSTSTIMMEABGEGEBEN = 1 WHERE WAHLTOKEN='{{TOKEN}}' AND ERSTSTIMMEABGEGEBEN = 0".replace("{{TOKEN}}", &stimme.token);
        let altered_rows = db_connection.dml(&query).unwrap();
        if altered_rows != 1 {
            return Err(BadRequest(Some("Token ungültig oder Erststimme bereits abgegeben! Es wurde keine Stimme abgegeben.")));
        }

        match erststimme {
            Erststimme::kandidat(k) => {
                // Ein Eintrag in der Tabelle ERSTSTIMME wird für den entsprechenden Kandidaten erstellt in dem Stimmkreis, in dem die Wahl abgegeben wurde
                let query = "INSERT INTO WIS.ERSTSTIMME (SELECT {{KANDIDAT}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{KANDIDAT}}", &k.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                db_connection.exec(&query).unwrap();
            }
            Erststimme::enthaltung => {
                // Bei einer Enthaltung wird einfach NULL anstelle eines Kandidaten eingefügt
                let query = "INSERT INTO WIS.ERSTSTIMME (SELECT NULL, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{TOKEN}}", &stimme.token);
                db_connection.exec(&query).unwrap();
            }
        }
    }

    if let Some(ref zweitstimme) = stimme.zweitstimme {
        // falls eine Zweitstimme mitgeliefert wurde, dann wird in der Datenbank das Token markiert als "Zweitstimme abgegeben"
        let query = "UPDATE WIS.WAHLTOKEN SET ZWEITSTIMMEABGEGEBEN = 1 WHERE WAHLTOKEN='{{TOKEN}}' AND ZWEITSTIMMEABGEGEBEN = 0".replace("{{TOKEN}}", &stimme.token);
        let altered_rows = db_connection.dml(&query).unwrap();
        if altered_rows != 1 {
            return Err(BadRequest(Some("Token ungültig oder Zweitstimme bereits abgegeben! Es wurde keine Zweitstimme abgegeben.")));
        }

        match zweitstimme {
            // Ein Eintrag in der Tabelle Zweitstimmekandidat wird für den entsprechenden Kandidaten erstellt in dem Stimmkreis, in dem die Wahl abgegeben wurde.
            Zweitstimme::kandidat(k) => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEKANDIDAT (SELECT {{KANDIDAT}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{KANDIDAT}}", &k.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                db_connection.exec(&query).unwrap();
            }
            // Bei der Zweitstimme kann auch eine Partei statt eines Kandidaten gewählt werden.
            Zweitstimme::partei(p) => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEPARTEI (SELECT {{PARTEI}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{PARTEI}}", &p.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                db_connection.exec(&query).unwrap();
            }
            // Auch hier kann man sich enthalten. Dabei wird ein NULL-Wert in die Tabelle ZWEITSTIMMEKANDIDAT eingefügt. Theoretisch könnte man es auch in ZWEITSTIMMEPARTEI einfügen.
            Zweitstimme::enthaltung => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEKANDIDAT (SELECT NULL, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{TOKEN}}", &stimme.token);
                db_connection.exec(&query).unwrap();
            }
        }
    }
    Ok("Stimme abgegeben")
}
