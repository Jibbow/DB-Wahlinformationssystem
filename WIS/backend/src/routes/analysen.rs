extern crate serde;
extern crate serde_json;

use rocket::State;
use rocket::response::content;


const ANALYSIS_CSU_AGE: &str = include_str!("../../queries/analysen/csu-sterberate.sql");
const ANALYSIS_FDP_INCOME: &str = include_str!("../../queries/analysen/fdp-gehalt.sql");


/// Vergleicht die Sterberate mit der Prozentualen Anzahl der CSU-Wähler
#[get("/analysen/csu-sterberate")]
pub fn csu_sterberate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PROZENT: f32,
        PARTEI: String,
        STERBERATE: f64,
    }

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(ANALYSIS_CSU_AGE)?.try_into()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}

/// Vergleicht das Durchschnittsgehalt mit der Prozentualen Anzahl der FDP-Wähler
#[get("/analysen/fdp-gehalt")]
pub fn fdp_gehalt(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>)
 -> Result<content::Json<String>, hdbconnect::HdbError> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        PROZENT: f32,
        PARTEI: String,
        GEHALT: u32,
    }

    let result: Vec<QueryResult> = db.get().expect("failed to connect to DB")
        .query(ANALYSIS_FDP_INCOME)?.try_into()?;
    Ok(content::Json(serde_json::to_string(&result).unwrap()))
}
