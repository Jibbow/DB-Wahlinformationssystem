## How to add a new Query+Endpoint

Load a file that contains the query by adding another line here in `routes.rs`:
```rust
// load sql queries during compile time
const TEST_QUERY: &str = include_str!("../queries/test.sql");
...
```
Create a new route ([documentation](https://github.com/SergioBenitez/Rocket)) in `routes.rs`:
```rust
#[get("/test")]
fn test() -> String {
    // define result from DB (names must match column names!)
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Result { SITZZAHL: usize, NAME: String, NR: usize }

    let result: Vec<Result> = get_db_connection().query(TEST_QUERY).unwrap().try_into().unwrap();
    serde_json::to_string(&result).unwrap()
}
```

Add your now route to the webserver in `routes![routes::test, ...]` in `main.rs`:
```rust
rocket::ignite().mount("/", routes![routes::test]).launch();
```


## Build and run the backend

Prerequisites:
 - install Rust (with rustup)
 - install Cargo (should be installed by rustup)
 - install the nightly toolchain

Now run:
```
cargo +nightly run
```
Webserver should be under [localhost:8000](localhost:8000)
