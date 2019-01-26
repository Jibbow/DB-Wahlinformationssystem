extern crate serde;
extern crate serde_json;
extern crate regex;

use rocket::State;
use rocket::response::content;
use rocket::http::Status;
use rocket::response::status::*;
use hdbconnect::HdbValue;


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

/// # Abstimmen
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
/// ```JSON
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
/// ```JSON
/// {
///  "token": "token-token-token-token",
///  "erststimme": {
///    "kandidat": 5
///  },
///  "zweitstimme": null
/// }
/// ```
#[post("/abstimmen", data = "<stimme>")]
pub fn abstimmen(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimme: rocket_contrib::json::Json<Stimmabgabe>)
 -> Result<&'static str, BadRequest<&'static str>> {
    // validate token
    let validator = regex::Regex::new(r"^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$").unwrap();
    if !validator.is_match(&stimme.token) {
        return Err(BadRequest(Some("Format des Tokens ist ungültig.")));
    }

    let mut connection = db.get().expect("failed to connect to DB");
    connection.set_auto_commit(false).unwrap();

    if let Some(ref erststimme) = stimme.erststimme {
        // falls eine Erststimme mitgeliefert wurde, dann wird in der Datenbank das Token markiert als "Erststimme abgegeben"
        let query = "UPDATE WIS.WAHLTOKEN SET ERSTSTIMMEABGEGEBEN = 1 WHERE WAHLTOKEN='{{TOKEN}}' AND ERSTSTIMMEABGEGEBEN = 0".replace("{{TOKEN}}", &stimme.token);
        let altered_rows = connection.dml(&query).unwrap();
        if altered_rows != 1 {
            connection.rollback().unwrap();
            return Err(BadRequest(Some("Token ungültig oder Erststimme bereits abgegeben! Es wurde keine Stimme abgegeben.")));
        }

        match erststimme {
            Erststimme::kandidat(k) => {
                // Ein Eintrag in der Tabelle ERSTSTIMME wird für den entsprechenden Kandidaten erstellt in dem Stimmkreis, in dem die Wahl abgegeben wurde
                let query = "INSERT INTO WIS.ERSTSTIMME (SELECT {{KANDIDAT}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{KANDIDAT}}", &k.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                connection.exec(&query).unwrap();
            }
            Erststimme::enthaltung => {
                // Bei einer Enthaltung wird einfach NULL anstelle eines Kandidaten eingefügt
                let query = "INSERT INTO WIS.ERSTSTIMME (SELECT NULL, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{TOKEN}}", &stimme.token);
                connection.exec(&query).unwrap();
            }
        }
    }

    if let Some(ref zweitstimme) = stimme.zweitstimme {
        // falls eine Zweitstimme mitgeliefert wurde, dann wird in der Datenbank das Token markiert als "Zweitstimme abgegeben"
        let query = "UPDATE WIS.WAHLTOKEN SET ZWEITSTIMMEABGEGEBEN = 1 WHERE WAHLTOKEN='{{TOKEN}}' AND ZWEITSTIMMEABGEGEBEN = 0".replace("{{TOKEN}}", &stimme.token);
        let altered_rows = connection.dml(&query).unwrap();
        if altered_rows != 1 {
            connection.rollback().unwrap();
            return Err(BadRequest(Some("Token ungültig oder Zweitstimme bereits abgegeben! Es wurde keine Zweitstimme abgegeben.")));
        }

        match zweitstimme {
            // Ein Eintrag in der Tabelle Zweitstimmekandidat wird für den entsprechenden Kandidaten erstellt in dem Stimmkreis, in dem die Wahl abgegeben wurde.
            Zweitstimme::kandidat(k) => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEKANDIDAT (SELECT {{KANDIDAT}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{KANDIDAT}}", &k.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                connection.exec(&query).unwrap();
            }
            // Bei der Zweitstimme kann auch eine Partei statt eines Kandidaten gewählt werden.
            Zweitstimme::partei(p) => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEPARTEI (SELECT {{PARTEI}}, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{PARTEI}}", &p.to_string())
                    .replace("{{TOKEN}}", &stimme.token);
                connection.exec(&query).unwrap();
            }
            // Auch hier kann man sich enthalten. Dabei wird ein NULL-Wert in die Tabelle ZWEITSTIMMEKANDIDAT eingefügt. Theoretisch könnte man es auch in ZWEITSTIMMEPARTEI einfügen.
            Zweitstimme::enthaltung => {
                let query = "INSERT INTO WIS.ZWEITSTIMMEKANDIDAT (SELECT NULL, T.STIMMKREIS , 2018 FROM WIS.WAHLTOKEN T WHERE T.WAHLTOKEN='{{TOKEN}}')"
                    .replace("{{TOKEN}}", &stimme.token);
                connection.exec(&query).unwrap();
            }
        }
    }
    connection.commit().unwrap();
    Ok("Stimme abgegeben")
}

/// # Tokeninfo
/// Gibt einige wichtige Informationen zu einem gegebenen Token zurück.
/// Das Token muss hierbei der Form einer UUIDv4 entsprechen:
/// 
/// `00000000-0000-0000-0000-000000000000`
/// 
/// ## Result
/// Das Ergebnis entspricht dem folgenden Format (JSON):
/// ```JSON
/// {
///   "WAHLTOKEN": "00000000-0000-0000-0000-000000000000",
///   "JAHR": 2018,
///   "STIMMKREIS": 101,
///   "ERSTSTIMMEABGEGEBEN": 0,
///   "ZWEITSTIMMEABGEGEBEN": 0
/// }
/// ```
/// Hierbei ist `ERSTSTIMMEABGEGEBEN`/`ZWEITSTIMMEABGEGEBEN` gleich `0`,
/// falls die jeweilige Stimme noch nicht abgegeben wurde, andernfalls `1`.
/// 
/// ## Error
/// Falls das Token ungültig ist, wird ein Fehler zurückgegeben (HTML: BadRequest).
#[get("/tokeninfo/<token>")]
pub fn tokeninfo(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, token: String)
 -> Result<content::Json<String>, Custom<String>> {
    // validate token
    let validator = regex::Regex::new(r"^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$").unwrap();
    if !validator.is_match(&token) {
        return Err(Custom(Status::BadRequest, format!("Format des Tokens ist ungültig: {}", token)));
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

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        "SELECT * FROM WIS.WAHLTOKEN WHERE WAHLTOKEN=?", 
        vec![HdbValue::STRING(token)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}

/// # Wahlzettel Erststimme
/// Diese Anfrage gibt den Wahlzettel für die Erststimme zurück.
/// Jeder Stimmkreis hat dabei eine eigene Auswahl an Kandidaten.
/// Der Stimmkreis wird hierbei normalerweise aus dem verwendeten
/// Token inferiert. Siehe dazu auch die Route [/tokeninfo/<token>](fn.tokeninfo.html).
/// 
/// ## Result
/// Das Ergebnis entspricht dem folgenden Format (JSON):
/// ```JSON
/// [
///   {
///     "PARTEI": "Christlich Soziale Union",
///     "PARTEI_ABKUERZUNG": "CSU",
///     "KANDIDAT_VORNAME": "Hans",
///     "KANDIDAT_NACHNAME": "Dieters",
///     "LISTENPOSITION": 1
///   },
///   ...
/// ]
/// ```
/// Hierbei entspricht die `LISTENPOSITION` der _eindeutigen_ Reihenfolge,
/// in der die Partei in _einem Stimmkreis_ aufgelistet ist.
/// 
/// Der Wahlzettel für die Erststimme ist für gewöhnlich sehr kurz,
/// da jede Partei nur maximal einen Kandidaten aufstellen darf.
#[get("/wahlzettel/erststimme/<stimmkreis>/<jahr>")]
pub fn wahlzettel_erststimme(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: i32, jahr: i32)
 -> Result<content::Json<String>, Custom<String>> {
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

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        STIMMZETTEL_ERSTSTIMME, 
        vec![HdbValue::INT(stimmkreis), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}

/// # Wahlzettel Zweitstimme
/// Diese Anfrage gibt den Wahlzettel für die Zweitstimme zurück.
/// Jeder Stimmkreis hat dabei eine eigene Auswahl an Kandidaten.
/// Der Stimmkreis wird hierbei normalerweise aus dem verwendeten
/// Token inferiert. Siehe dazu auch die Route [/tokeninfo/<token>](fn.tokeninfo.html).
/// 
/// ## Result
/// Das Ergebnis entspricht dem folgenden Format (JSON):
/// ```JSON
/// [
///   {
///     "PARTEI": "Christlich Soziale Union",
///     "PARTEI_ABKUERZUNG": "CSU",
///     "KANDIDAT_VORNAME": "Hans",
///     "KANDIDAT_NACHNAME": "Dieters",
///     "LISTENPOSITION": 1
///   },
///   ...
/// ]
/// ```
/// Hierbei entspricht die `LISTENPOSITION` der _eindeutigen_ Reihenfolge,
/// in der die Kandidaten für _eine Partei_ in _einem Stimmkreis_ aufgestellt sind.
/// 
/// Der Wahlzettel für die Zweitstimme ist oft sehr lang (mehrere 100 Einträge) für
/// einen Stimmkreis, da jede Partei beliebig viele Kandidaten aufstellen darf.
#[get("/wahlzettel/zweitstimme/<stimmkreis>/<jahr>")]
pub fn wahlzettel_zweitstimme(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, stimmkreis: i32, jahr: i32)
 -> Result<content::Json<String>, Custom<String>> {
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

    let mut connection = db.get().expect("failed to connect to DB");
    let result = super::query_database::<QueryResult>(&mut connection, 
        STIMMZETTEL_ZWEITSTIMME, 
        vec![HdbValue::INT(stimmkreis), HdbValue::INT(jahr)]);
    match result {
        Ok(r) => Ok(content::Json(serde_json::to_string(&r).unwrap())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Error while processing query: {}", e)))
    }
}
