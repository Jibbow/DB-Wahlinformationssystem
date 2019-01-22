extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;


/// Gibt eine Liste aller Parteien bei der Landtagswahl zurück.
#[get("/parteien")]
pub fn parteien(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ID: u32,
        NAME: String,
        ABKUERZUNG: String,
        FARBE: String,
    }
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query("SELECT ID, ABKUERZUNG, NAME, FARBE FROM WIS.PARTEI")?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}

/// Gibt eine Liste aller Stimmkreise in Bayern zurück.
/// Vorsicht: die IDs der Stimmkreise ändern sich über die Jahre hinweg!
#[get("/stimmkreise/<jahr>")]
pub fn stimmkreise(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>, jahr: u32)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
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

    let query = "SELECT S.NR, S.NAME, S.STIMMBERECHTIGTE, W.NAME AS WAHLKREIS, W.NR AS WAHLKREISNR
                 FROM WIS.STIMMKREIS S JOIN WIS.WAHLKREIS W ON S.WAHLKREIS=W.NR
                 WHERE S.JAHR={{JAHR}}"
        .replace("{{JAHR}}", &jahr.to_string());
    let mut connection = db.get().expect("failed to connect to DB");
    let result: Vec<QueryResult> = connection.query(&query)?.try_into()?;
    connection.commit()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}
