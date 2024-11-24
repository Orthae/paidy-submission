use paidy_submission::infrastructure::connection_factory::{
    DatabaseConfiguration, PostgresConnectionPoolFactory,
};
use sqlx::Row;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

#[tokio::test]
async fn should_create_pool() {
    let container = Postgres::default()
        .with_db_name("test")
        .with_user("root")
        .with_password("qwerty")
        .start()
        .await
        .expect("Failed to create PostgreSQL container");

    let config = DatabaseConfiguration {
        host: "localhost".to_string(),
        port: container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port"),
        database: "test".to_string(),
        username: "root".to_string(),
        password: "qwerty".to_string(),
    };

    let pool = PostgresConnectionPoolFactory::create(config).await;

    let result: i64 = sqlx::query("SELECT COUNT(*) FROM items")
        .fetch_one(&pool)
        .await
        .expect("Failed count query")
        .get(0);

    assert_eq!(0, result);
}
