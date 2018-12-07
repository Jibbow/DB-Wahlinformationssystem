extern crate serde;
extern crate serde_json;
extern crate handlebars;

use super::get_db_connection;

// load sql queries during compile time
const TEST_QUERY: &str = include_str!("../queries/test.sql");


#[get("/test")]
pub fn test() -> String {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { SITZZAHL: usize, NAME: String, NR: usize }

    let result: Vec<Result> = get_db_connection().query(TEST_QUERY).unwrap().try_into().unwrap();
    serde_json::to_string(&result).unwrap()
}
