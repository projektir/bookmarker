use diesel::r2d2::PoolError;
use thiserror::Error;

pub mod bookmark;
pub mod schema;
pub mod unit;

pub type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("{0}")]
    PoolError(#[from] r2d2::Error),
    #[error("{0}")]
    OperationError(#[from] diesel::result::Error),
    #[error("{0}")]
    ConnectionError(#[from] diesel::ConnectionError),
    #[error("{0}")]
    MigrationError(#[from] diesel_migrations::RunMigrationsError),
}

fn init_pool(database_url: &str, size: u8) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().max_size(size.into()).build(manager)
}

pub fn establish_connection(url: &str, pool_size: u8) -> Result<Pool, PoolError> {
    init_pool(&url, pool_size)
}
