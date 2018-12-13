Backend for WIS
=====

## Build Docker image
The backend can also be packed into a highly optimized docker image. Just install docker and run the following command inside the `backend/` directory:
```
docker build -t wis-backend .
```
Start the backend with:
```
docker run -p 8000:8000 --env-file ../.env wis-backend
```
You can reach the backend under [localhost:8000](localhost:8000)

## Build and run the backend without Docker
If you don't want to use docker, you can also build and run the backend directly on your machine.

Prerequisites:
 - install Rust (with rustup)
 - install Cargo (should be installed by rustup)
 - install the nightly toolchain

Now run:
```
cargo +nightly run
```
Webserver should be under [localhost:8000](localhost:8000)


## How to: Add a new Query+Endpoint

Load a file that contains the query by adding another line here in `routes.rs`:
```rust
// load sql queries during compile time
const TEST_QUERY: &str = include_str!("../queries/test.sql");
...
```
Create a new route ([documentation](https://github.com/SergioBenitez/Rocket)) in `routes.rs`:
```rust
#[get("/test")]
pub fn ueberhangmandate(db: State<r2d2::Pool<hdbconnect::ConnectionManager>>) -> content::Json<String> {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct QueryResult {
        ...
    }

    let result: Vec<QueryResult> = db.get().unwrap()
        .query(QUERY).unwrap().try_into().unwrap();
    content::Json(serde_json::to_string(&result).unwrap())
}
```

Add your now route to the webserver in `routes![routes::test, ...]` in `main.rs`:
```rust
rocket::ignite().mount("/", routes![routes::test]).launch();
```
Done!

## Tipp: How to quickly test new queries
The directory "`queries/`" contains a python script "`query.py`" which runs a specified query against the database specified in the project's `.env` file (=zero setup).

Simple write your query into a file and pass it as an argument to the script. Example:
```
python query.py --file test.sql
```
Alternatively, you can also pass a query directly as a parameter to the script:
```
python query.py --sql "select * from WIS.PARTEI"
```

The result of the query is printed to STDOUT.
