use paidy_submission::domain::repository::ItemRepository;
use paidy_submission::infrastructure::connection_factory::{
    DatabaseConfiguration, PostgresConnectionPoolFactory,
};
use paidy_submission::infrastructure::item_repository::ItemRepositoryImpl;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

mod repository_tests {
    use super::*;
    use paidy_submission::domain::item_factory::ItemFactory;

    struct RepositoryTestContext {
        repository: ItemRepositoryImpl,
        _container: ContainerAsync<Postgres>,
    }

    impl RepositoryTestContext {
        pub async fn create_test_context() -> RepositoryTestContext {
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

            let pool = PostgresConnectionPoolFactory::new(config).await;
            let repository = ItemRepositoryImpl::new(pool);

            RepositoryTestContext {
                repository,
                _container: container,
            }
        }
    }

    #[tokio::test]
    async fn delete_unexisting_item() {
        let context = RepositoryTestContext::create_test_context().await;
        let item = ItemFactory::try_create(1, "Pierogi".to_string())
            .expect("Failed to create item");

        context
            .repository
            .delete_item(&item.id)
            .await
            .expect("Failed to delete item");
    }

    #[tokio::test]
    async fn delete_existing_item() {
        let context = RepositoryTestContext::create_test_context().await;
        let item = ItemFactory::try_create(1, "Pierogi".to_string())
            .expect("Failed to create item");

        context
            .repository
            .save_items(&vec![item.clone()])
            .await
            .expect("Failed to save item");

        context
            .repository
            .delete_item(&item.id)
            .await
            .expect("Failed to delete item");

        let query_result = context
            .repository
            .find_item(&item.id)
            .await
            .expect("Failed to find item");

        assert!(query_result.is_none());
    }

    #[tokio::test]
    async fn create_item() {
        let context = RepositoryTestContext::create_test_context().await;

        let item = ItemFactory::try_create(1, "Pierogi".to_string())
            .expect("Failed to create item");

        context
            .repository
            .save_items(&vec![item.clone()])
            .await
            .expect("Failed to save item");

        let saved = context
            .repository
            .find_item(&item.id)
            .await
            .expect("Failed to find item")
            .expect("Failed to find saved item");

        assert_eq!(item, saved);
    }

    #[tokio::test]
    async fn create_items_transaction_fail() {
        let context = RepositoryTestContext::create_test_context().await;
        let item = ItemFactory::try_create(1, "Pierogi".to_string())
            .expect("Failed to create item");

        let save_result = context
            .repository
            .save_items(&vec![item.clone(), item.clone()])
            .await;

        assert!(save_result.is_err());

        let query_result = context
            .repository
            .find_item(&item.id)
            .await
            .expect("Failed to find item");

        assert!(query_result.is_none());
    }

    #[tokio::test]
    async fn create_items_transaction_same_table() {
        let context = RepositoryTestContext::create_test_context().await;
        let first_item = ItemFactory::try_create(1, "Pierogi".to_string())
            .expect("Failed to create item");
        let second_item = ItemFactory::try_create(1, "Schabowy".to_string())
            .expect("Failed to create item");

        context
            .repository
            .save_items(&vec![first_item.clone(), second_item.clone()])
            .await
            .expect("Failed to save items");

        let query_result = context
            .repository
            .find_items_by_table(&first_item.table_id)
            .await
            .expect("Failed to find item");

        assert_eq!(2, query_result.len());
        assert_eq!(first_item, query_result[0]);
        assert_eq!(second_item, query_result[1]);
    }

    #[tokio::test]
    async fn create_items_different_tables() {
        let context = RepositoryTestContext::create_test_context().await;
        let first_item = ItemFactory::try_create(1,"Pierogi".to_string())
            .expect("Failed to create item");
        let second_item = ItemFactory::try_create(2, "Schabowy".to_string())
            .expect("Failed to create item");

        context
            .repository
            .save_items(&vec![first_item.clone()])
            .await
            .expect("Failed to save items");

        context
            .repository
            .save_items(&vec![second_item.clone()])
            .await
            .expect("Failed to save items");

        let first_table_query = context
            .repository
            .find_items_by_table(&first_item.table_id)
            .await
            .expect("Failed to find item");

        assert_eq!(1, first_table_query.len());
        assert_eq!(first_item, first_table_query[0]);

        let second_table_query = context
            .repository
            .find_items_by_table(&second_item.table_id)
            .await
            .expect("Failed to find item");

        assert_eq!(1, second_table_query.len());
        assert_eq!(second_item, second_table_query[0]);
    }
}
