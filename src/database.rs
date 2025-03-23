use crate::error_handler::CustomError;
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2;
use std::{env, error::Error};
use url::form_urlencoded;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

lazy_static! {
    static ref POOL: Pool = {
        let db_host = env::var("DATABASE_HOST").expect("Database host not set");
        let db_user = env::var("DATABASE_USER").expect("Database user not set");
        let db_password = env::var("DATABASE_PASSWORD").expect("Database password not set");
        let db_database = env::var("DATABASE_DATABASE").expect("Database name not set");
        let db_port = env::var("DATABASE_PORT")
            .or::<Result<String, &str>>(Ok("5432".to_string()))
            .unwrap();
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user,
            form_urlencoded::byte_serialize(db_password.as_bytes()).collect::<String>(),
            db_host,
            db_port,
            db_database
        );
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");
    run_migrations(&mut conn).unwrap();
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
