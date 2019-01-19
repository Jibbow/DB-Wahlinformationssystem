extern crate serde;
extern crate serde_json;
extern crate regex;

use rocket::State;
use rocket::response::content;
use rocket::response::status::*;


const STIMMZETTEL_ERSTSTIMME: &str = include_str!("../../queries/stimmabgabe/stimmzettel-erststimme.sql");
const STIMMZETTEL_ZWEITSTIMME: &str = include_str!("../../queries/stimmabgabe/stimmzettel-zweitstimme.sql");



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