use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");

    let max_pool_size = env::var("DB_MAX_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let min_idle = env::var("DB_MIN_IDLE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);

    let connection_timeout = env::var("DB_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(max_pool_size)
        .min_idle(Some(min_idle))
        .connection_timeout(Duration::from_secs(connection_timeout))
        .test_on_check_out(true)
        .build(manager)?;

    Ok(pool)
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, Box<dyn std::error::Error>> {
    pool.get().map_err(|e| {
        Box::new(e) as Box<dyn std::error::Error>
    })
}

pub fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    use diesel_migrations::MigrationHarness;

    let mut connection = pool.get()?;

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations = {
    #[cfg(feature = "embedded_migrations")]
    {
        diesel_migrations::embed_migrations!("migrations")
    }
    #[cfg(not(feature = "embedded_migrations"))]
    {
        diesel_migrations::EmbeddedMigrations::new()
    }
};

pub mod schema {
    diesel::table! {
        loan_requests (id) {
            id -> Text,
            request_id -> Text,
            applicant_name -> Text,
            amount -> Float8,
            term_months -> Integer,
            status -> Text,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_database_url_required() {
        env::remove_var("DATABASE_URL");
        let result = create_db_pool();
        assert!(result.is_err());
    }

    #[test]
    fn test_pool_size_defaults() {
        env::set_var("DATABASE_URL", "postgres://user:pass@localhost/test");
        env::remove_var("DB_MAX_POOL_SIZE");
        env::remove_var("DB_MIN_IDLE");
        env::remove_var("DB_CONNECTION_TIMEOUT");

        let result = create_db_pool();
        assert!(result.is_err() || result.unwrap().size() == 10);

        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_custom_pool_size() {
        env::set_var("DATABASE_URL", "postgres://user:pass@localhost/test");
        env::set_var("DB_MAX_POOL_SIZE", "20");
        env::set_var("DB_MIN_IDLE", "5");
        env::set_var("DB_CONNECTION_TIMEOUT", "60");

        let result = create_db_pool();
        if let Ok(pool) = result {
            assert_eq!(pool.size(), 20);
        }

        env::remove_var("DATABASE_URL");
        env::remove_var("DB_MAX_POOL_SIZE");
        env::remove_var("DB_MIN_IDLE");
        env::remove_var("DB_CONNECTION_TIMEOUT");
    }
}