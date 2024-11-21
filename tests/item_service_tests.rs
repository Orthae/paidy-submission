use mockall::predicate::eq;
use paidy_submission::application::item_service::{ItemService, ItemServiceImpl};
use paidy_submission::domain::item_factory::ItemFactory;
use paidy_submission::domain::repository::MockItemRepository;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn delete_item_test() {
    let item_id = Uuid::now_v7();
    let mut repository = MockItemRepository::new();
    repository.expect_delete_item()
        .with(eq(item_id))
        .return_const(Ok(()))
        .once();
    
    let service = ItemServiceImpl::new(Arc::new(repository));

    service.delete_item(item_id)
        .await
        .expect("Failed to delete item");
}

#[tokio::test]
async fn get_item_test() {
    let item = ItemFactory::try_create(1, "name".to_string()) 
        .expect("Failed to create item");
    
    let mut repository = MockItemRepository::new();
    repository.expect_find_item()
        .return_const(Ok(Some(item.clone())))
        .with(eq(item.id))
        .once();
    
    let service = ItemServiceImpl::new(Arc::new(repository));

    let result = service.get_item(item.id)
        .await
        .expect("Failed to get item")
        .expect("Returned empty item");
    
    assert_eq!(result.id, item.id);
    assert_eq!(result.table_id, item.table_id);
    assert_eq!(result.name, item.name);
    assert_eq!(result.preparation_time, item.preparation_time);
}