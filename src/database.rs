use crate::error::ApiError;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn establish_connection(database_url: &str) -> Result<PgConnection, ApiError> {
  match PgConnection::establish(&database_url) {
    Ok(connection) => Ok(connection),
    Err(error_msg) => Err(format!("{:?}", error_msg).into()),
  }
}

pub struct Database {
  connection: PgConnection,
}

impl Database {
  pub fn try_new(database_url: &str) -> Result<Self, ApiError> {
    let connection = establish_connection(database_url)?;
    log::info!("Database connection established.");
    Ok(Database { connection })
  }
}
