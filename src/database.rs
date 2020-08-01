use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Handles connections to the database. Clones of this handle share the same
/// underlying connection pool.
#[derive(Clone)]
pub struct Database {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl Database {
    /// Creates a `Database` connected to the provided URL.
    pub fn connect(url: impl Into<String>) -> anyhow::Result<Self> {
        let manager = ConnectionManager::new(url);
        Ok(Database {
            pool: Arc::new(Pool::new(manager)?),
        })
    }

    /// Gets a connection to this database.
    pub fn connection(&self) -> anyhow::Result<DbConnection> {
        self.pool.get().map_err(Into::into)
    }

    /// Starts a transaction using a connection to this database. The provided function
    /// will be called with that connection.
    pub fn transaction<T, F>(&self, transaction: F) -> anyhow::Result<T>
    where
        F: FnOnce(&DbConnection) -> anyhow::Result<T>,
    {
        let conn = self.connection()?;
        conn.transaction(|| transaction(&conn).map_err(Into::into))
    }
}
