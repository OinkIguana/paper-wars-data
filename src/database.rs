use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

/// Handles connections to the database.
pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    /// Creates a `Database` connected to the provided URL.
    pub fn connect(url: impl Into<String>) -> anyhow::Result<Self> {
        let manager = ConnectionManager::new(url);
        Ok(Database {
            pool: Pool::new(manager)?,
        })
    }

    /// Gets a connection to this database.
    pub fn connection(&self) -> anyhow::Result<Conn> {
        self.pool.get().map_err(Into::into)
    }

    /// Starts a transaction using a connection to this database. The provided function
    /// will be called with that connection.
    pub fn transaction<T, E, F>(&self, proc: F) -> anyhow::Result<T>
    where
        E: 'static + Sync + Send + std::error::Error,
        F: Fn(&Conn) -> Result<T, E>,
    {
        let conn = self.connection()?;
        conn.transaction(|| proc(&conn).map_err(Into::into))
    }
}
