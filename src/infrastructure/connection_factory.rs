use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

pub struct DatabaseConfiguration {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub struct PostgresConnectionPoolFactory;

impl PostgresConnectionPoolFactory {
    pub async fn create(config: DatabaseConfiguration) -> Pool<Postgres> {
        let connection_options = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .database(&config.database)
            .username(&config.username)
            .password(&config.password);

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect_with(connection_options)
            .await
            .expect("Failed to create pool");

        sqlx::migrate!("./migrations/")
            .run(&pool)
            .await
            .expect("Failed to migrate database");

        pool
    }
}
