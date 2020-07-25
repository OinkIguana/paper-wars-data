use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

type PgConnectionManager = ConnectionManager<PgConnection>;

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect(url: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ConnectionManager::new(url);
        Ok(Database { pool: Pool::new(manager)? })
    }

    pub fn connection(&self) -> Result<PooledConnection<PgConnectionManager>, Box<dyn std::error::Error>> {
        self.pool.get().map_err(Into::into)
    }
}
