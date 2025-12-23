pub use diesel::ConnectionError;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let conn = PgConnection::establish(&db_url)?;

        Ok(Self { conn })
    }
}
