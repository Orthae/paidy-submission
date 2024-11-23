use mockall::predicate::eq;
use paidy_submission::application::item_service::{ApplicationError, CreateItemModel, CreateItemsCommand, ItemService, ItemServiceImpl};
use paidy_submission::domain::repository::{MockItemRepository, RepositoryError};
use std::sync::Arc;
use uuid::Uuid;
use paidy_submission::domain::item::Item;

mod item_service_tests {
    use super::*;
    
    mod delete_item_tests {
        use paidy_submission::domain::item_factory::MockItemFactory;
        use super::*;
        #[tokio::test]
        async fn should_delete_item() {
            let table_id = 1;
            let item_id = Uuid::now_v7();
            let mut repository = MockItemRepository::new();
            repository.expect_delete_item()
                .with(eq(table_id), eq(item_id))
                .return_const(Ok(()))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            service.delete_item(table_id, item_id)
                .await
                .expect("Failed to delete item");
        }

        #[tokio::test]
        async fn should_handle_repository_error() {
            let table_id = 1;
            let item_id = Uuid::now_v7();
            let mut repository = MockItemRepository::new();
            repository.expect_delete_item()
                .with(eq(table_id), eq(item_id))
                .return_const(Err(RepositoryError::InternalRepositoryError("Crash".to_string())))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.delete_item(table_id, item_id)
                .await
                .expect_err("Repository delete did not fail");

            assert_eq!(result, ApplicationError::InternalError);
        }
    }
    
    mod get_item_tests {
        use chrono::Utc;
        use paidy_submission::domain::item::Item;
        use paidy_submission::domain::item_factory::MockItemFactory;
        use super::*;
        
        #[tokio::test]
        async fn should_get_item() {
            let item = Item::try_new(Uuid::now_v7(), 1, "name".to_string(), Utc::now())
                .expect("Failed to create item");

            let mut repository = MockItemRepository::new();
            repository.expect_find_item()
                .return_const(Ok(Some(item.clone())))
                .with(eq(item.table_id), eq(item.id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_item(item.table_id, item.id)
                .await
                .expect("Failed to get item");

            assert_eq!(result.id, item.id);
            assert_eq!(result.table_id, item.table_id);
            assert_eq!(result.name, item.name);
            assert_eq!(result.preparation_time, item.preparation_time);
        }

        #[tokio::test]
        async fn should_get_resource_not_found_on_missing_item() {
            let item = Item::try_new(Uuid::now_v7(), 1, "name".to_string(), Utc::now())
                .expect("Failed to create item");

            let mut repository = MockItemRepository::new();
            repository.expect_find_item()
                .return_const(Ok(None))
                .with(eq(item.table_id), eq(item.id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_item(item.table_id, item.id)
                .await
                .expect_err("Get item did not fail");

            assert_eq!(result, ApplicationError::ResourceNotFound);
        }

        #[tokio::test]
        async fn should_handle_repository_error() {
            let item = Item::try_new(Uuid::now_v7(), 1, "name".to_string(), Utc::now())
                .expect("Failed to create item");

            let mut repository = MockItemRepository::new();
            repository.expect_find_item()
                .return_const(Err(RepositoryError::InternalRepositoryError("Crash".to_string())))
                .with(eq(item.table_id), eq(item.id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_item(item.table_id, item.id)
                .await
                .expect_err("Get item did not fail");

            assert_eq!(result, ApplicationError::InternalError);
        }
    }
    
    mod get_items_tests {
        use chrono::Utc;
        use paidy_submission::domain::item::Item;
        use paidy_submission::domain::item_factory::MockItemFactory;
        use super::*;
        
        #[tokio::test]
        async fn should_get_multiple_items() {
            let table_id = 1;
            let first_item = Item::try_new(Uuid::now_v7(), 1, "first".to_string(), Utc::now())
                .expect("Failed to create item");

            let second_item = Item::try_new(Uuid::now_v7(), 1, "second".to_string(), Utc::now())
                .expect("Failed to create item");

            let mut repository = MockItemRepository::new();
            repository.expect_find_items_by_table()
                .return_const(Ok(vec![first_item.clone(), second_item.clone()]))
                .with(eq(table_id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_items(table_id)
                .await
                .expect("Failed to get items");

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].id, first_item.id);
            assert_eq!(result[0].table_id, first_item.table_id);
            assert_eq!(result[0].name, first_item.name);
            assert_eq!(result[0].preparation_time, first_item.preparation_time);

            assert_eq!(result[1].id, second_item.id);
            assert_eq!(result[1].table_id, second_item.table_id);
            assert_eq!(result[1].name, second_item.name);
            assert_eq!(result[1].preparation_time, second_item.preparation_time);
        }

        #[tokio::test]
        async fn should_get_single_items() {
            let item = Item::try_new(Uuid::now_v7(), 1, "name".to_string(), Utc::now())
                .expect("Failed to create item");

            let mut repository = MockItemRepository::new();
            repository.expect_find_items_by_table()
                .return_const(Ok(vec![item.clone()]))
                .with(eq(item.table_id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_items(item.table_id)
                .await
                .expect("Failed to get items");

            assert_eq!(result.len(), 1);
            assert_eq!(result[0].id, item.id);
            assert_eq!(result[0].table_id, item.table_id);
            assert_eq!(result[0].name, item.name);
            assert_eq!(result[0].preparation_time, item.preparation_time);
        }

        #[tokio::test]
        async fn should_handle_repository_error() {
            let table_id = 1;

            let mut repository = MockItemRepository::new();
            repository.expect_find_items_by_table()
                .return_const(Err(RepositoryError::InternalRepositoryError("Crash".to_string())))
                .with(eq(table_id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_items(table_id)
                .await
                .expect_err("Get items did not fail");

            assert_eq!(result, ApplicationError::InternalError);
        }

        #[tokio::test]
        async fn should_get_empty_list_on_missing_items() {
            let table_id = 1;

            let mut repository = MockItemRepository::new();
            repository.expect_find_items_by_table()
                .return_const(Ok(vec![]))
                .with(eq(table_id))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));

            let result = service.get_items(table_id)
                .await
                .expect("Failed to get items");

            assert!(result.is_empty());
        }
    }
    
    mod create_items_test {
        use chrono::{Duration, Utc};
        use paidy_submission::domain::item::{Item, ItemValidationError};
        use paidy_submission::domain::item_factory::MockItemFactory;
        use super::*;
        
        #[tokio::test]
        async fn should_create_single_item() {
            let table_id = 1;
            let item = Item::try_new(Uuid::now_v7(), table_id, "name".to_string(), Utc::now())
                .expect("Failed to create item");
            
            let mut factory = MockItemFactory::new();
            factory.expect_try_create()
                .with(eq(item.table_id), eq(item.name.clone()))
                .return_const(Ok(item.clone()))
                .once();
            
            let mut repository = MockItemRepository::new();
            repository.expect_save_items()
                .with(eq(vec![item.clone()]))
                .return_const(Ok(()))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(factory));
            let command = CreateItemsCommandExt::from_item(item.clone());

            let result = service.create_items(table_id, command)
                .await
                .expect("Failed to save items");
            
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].id, item.id);
            assert_eq!(result[0].table_id, item.table_id);
            assert_eq!(result[0].name, item.name);
            assert_eq!(result[0].preparation_time, item.preparation_time);
        }

        #[tokio::test]
        async fn should_save_multiple_items() {
            let table_id = 1;
            let first_item = Item::try_new(Uuid::now_v7(), table_id, "first".to_string(), Utc::now())
                .expect("Failed to create item");

            let second_item = Item::try_new(Uuid::now_v7(), table_id, "second".to_string(), Utc::now() + Duration::minutes(5))
                .expect("Failed to create item");

            let mut factory = MockItemFactory::new();
            factory.expect_try_create()
                .with(eq(first_item.table_id), eq(first_item.name.clone()))
                .return_const(Ok(first_item.clone()))
                .once();

            factory.expect_try_create()
                .with(eq(second_item.table_id), eq(second_item.name.clone()))
                .return_const(Ok(second_item.clone()))
                .once();

            let mut repository = MockItemRepository::new();
            repository.expect_save_items()
                .with(eq(vec![first_item.clone(), second_item.clone()]))
                .return_const(Ok(()))
                .once();

            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(factory));
            let command = CreateItemsCommandExt::from_items(vec![first_item.clone(), second_item.clone()]);

            let result = service.create_items(table_id, command)
                .await
                .expect("Failed to save items");

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].id, first_item.id);
            assert_eq!(result[0].table_id, first_item.table_id);
            assert_eq!(result[0].name, first_item.name);
            assert_eq!(result[0].preparation_time, first_item.preparation_time);
            
            assert_eq!(result[1].id, second_item.id);
            assert_eq!(result[1].table_id, second_item.table_id);
            assert_eq!(result[1].name, second_item.name);
            assert_eq!(result[1].preparation_time, second_item.preparation_time);
        }
        
        #[tokio::test]
        async fn should_handle_validation_error() {
            let invalid_command = CreateItemsCommandExt::from_items(vec![]);
            
            let mut repository = MockItemRepository::new();
            repository.expect_delete_item()
                .never();
            
            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(MockItemFactory::new()));
            
            let result = service.create_items(1, invalid_command)
                .await
                .expect_err("Validation did not fail");
            
            assert!(matches!(result, ApplicationError::ValidationError(_)));
        }
        
        #[tokio::test]
        async fn should_handle_factory_error() {
            let mut factory = MockItemFactory::new();
            factory.expect_try_create()
                .return_const(Err(ItemValidationError::EmptyName))
                .once();
            
            let mut repository = MockItemRepository::new();
            repository.expect_save_items()
                .never();
            
            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(factory));
            let command = CreateItemsCommandExt::default_invalid();
            
            let result = service.create_items(1, command)
                .await
                .expect_err("Factory did not fail");

            assert!(matches!(result, ApplicationError::ValidationError(_)));
        }
        
        #[tokio::test]
        async fn should_handle_repository_error() {
            let table_id = 1;
            let item = Item::try_new(Uuid::now_v7(), table_id, "name".to_string(), Utc::now())
                .expect("Failed to create item");
            
            let mut factory = MockItemFactory::new();
            factory.expect_try_create()
                .with(eq(item.table_id), eq(item.name.clone()))
                .return_const(Ok(item.clone()))
                .once();
            
            let mut repository = MockItemRepository::new();
            repository.expect_save_items()
                .with(eq(vec![item.clone()]))
                .return_const(Err(RepositoryError::InternalRepositoryError("Crash".to_string())))
                .once();
            
            let service = ItemServiceImpl::new(Arc::new(repository), Arc::new(factory));
            let command = CreateItemsCommandExt::from_item(item.clone());
            
            let result = service.create_items(table_id, command)
                .await
                .expect_err("Repository did not fail");
            
            assert_eq!(result, ApplicationError::InternalError);
        }
    }
}

struct CreateItemsCommandExt;

impl CreateItemsCommandExt {
    
    pub fn default_invalid() -> CreateItemsCommand {
        CreateItemsCommand {
            items: vec![CreateItemModel { name: "".to_string() }]
        }
    }

    pub fn from_item(item: Item) -> CreateItemsCommand {
        CreateItemsCommand {
            items: vec![CreateItemModel { name: item.name }]
        }
    }
    
    pub fn from_items(items: Vec<Item>) -> CreateItemsCommand {
        CreateItemsCommand {
            items: items
                .into_iter()
                .map(|item| CreateItemModel { name: item.name })
                .collect()
        }
    }
}









