use sqlx::{
    migrate::{MigrateError, Migrator},
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub type DbPool = PgPool;

pub async fn connect(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let options = database_url
        .parse::<PgConnectOptions>()?
        .statement_cache_capacity(0);

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), MigrateError> {
    MIGRATOR.run(pool).await
}
