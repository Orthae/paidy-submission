use crate::infrastructure::connection_factory::DatabaseConfiguration;

pub trait Load<T> {
    fn load() -> T;
}

impl Load<DatabaseConfiguration> for DatabaseConfiguration {
    fn load() -> DatabaseConfiguration {
        DatabaseConfiguration {
            host: std::env::var("PAIDY_DB_HOST")
                .expect("PAIDY_DB_HOST must be set"),
            port: std::env::var("PAIDY_DB_PORT")
                .expect("PAIDY_DB_PORT must be set")
                .parse()
                .expect("PAIDY_DB_PORT must be a number"),
            database: std::env::var("PAIDY_DB_NAME")
                .expect("PAIDY_DB_NAME must be set"),
            username: std::env::var("PAIDY_DB_USER")
                .expect("PAIDY_DB_USER must be set"),
            password: std::env::var("PAIDY_DB_PASSWORD")
                .expect("PAIDY_DB_PASSWORD must be set"),
        }
    }
}