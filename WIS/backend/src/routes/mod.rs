pub mod analysen;
pub mod bayern;
pub mod data;
pub mod other;
pub mod stimmabgabe;
pub mod stimmkreis;

#[derive(Serialize)]
struct DbResult<T> {
    processing_time: i32,
    data: Vec<T>,
}

/// # Query Database
/// Shorthand-function for querying the database for a resultset.
/// This function internally uses prepared statements for safety.
/// Additionally, it provides the caller with info about the database execution time.
fn query_database<'de, T>(connection: &mut hdbconnect::Connection, query: &str, params: Vec<hdbconnect::HdbValue>)
 -> Result<DbResult<T>, hdbconnect::HdbError> 
where
        T: serde::de::Deserialize<'de>,
{
    // prepared statements for safety
    let mut prepared_statement = connection.prepare(query)?;
    prepared_statement.add_row_to_batch(params)?;

    // query database
    let data: Vec<T> = prepared_statement.execute_batch()?.into_resultset()?.try_into()?;
    let processing_time = connection.get_server_resource_consumption_info()?.server_proc_time;

    connection.commit()?;

    Ok(DbResult {
        processing_time,
        data
    })
}
