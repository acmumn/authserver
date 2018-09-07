//! The database and related types.

mod schema;

use std::sync::Arc;

use diesel::{
    self,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use futures::{
    future::{err, poll_fn, Either},
    prelude::*,
};
use tokio_threadpool::blocking;

use db::schema::{identity_jwt_escrow, members_members};
use {Error, ErrorKind, Result};

/// A pool of connections to the database.
#[derive(Clone)]
pub struct DB {
    pool: Arc<Pool<ConnectionManager<MysqlConnection>>>,
}

impl DB {
    /// Connects to the database with the given number of connections.
    pub fn connect(database_url: &str) -> Result<DB> {
        let pool = Arc::new(Pool::new(ConnectionManager::new(database_url))?);
        Ok(DB { pool })
    }

    fn async_query<E, F, T>(&self, func: F) -> impl Future<Item = T, Error = Error>
    where
        E: Into<Error>,
        F: Fn(&MysqlConnection) -> ::std::result::Result<T, E>,
    {
        match self.pool.get() {
            Ok(conn) => Either::A(
                poll_fn(move || {
                    blocking(|| func(&*conn).map_err(|e| e.into())).map_err(|_| {
                        panic!("Database queries must be run inside a Tokio thread pool!")
                    })
                }).and_then(|r| r),
            ),
            Err(e) => Either::B(err(e.into())),
        }
    }
}
