extern crate serde;
extern crate serde_json;
extern crate handlebars;

use super::get_db_connection;
use rocket::response::content;

// load sql queries during compile time
const TEST_QUERY: &str = include_str!("../queries/test.sql");
const SITZVERTEILUNG_QUERY: &str = include_str!("../queries/sitzverteilung.sql");
const LANDTAGSMITGLIEDER_QUERY: &str = include_str!("../queries/landtagsmitglieder.sql");


#[get("/test")]
pub fn test() -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { SITZZAHL: u32, NAME: String, NR: u32 }

    let result: Vec<Result> = get_db_connection().query(TEST_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


#[get("/sitzverteilung/<jahr>")]
pub fn sitzverteilung(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { PARTEI: String, SITZE: u32 }

    let result: Vec<Result> = get_db_connection().query(SITZVERTEILUNG_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


#[get("/landtagsmitglieder/<jahr>")]
pub fn landtagsmitglieder(jahr: u32) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { VORNAME: String, NACHNAME: String, PARTEI: String }

    let result: Vec<Result> = get_db_connection().query(LANDTAGSMITGLIEDER_QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}


// MOCK!!!!!!!
#[get("/stimmverteilung/<jahr>")]
pub fn stimmverteilung(jahr: u32) -> content::Json<String> {
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
