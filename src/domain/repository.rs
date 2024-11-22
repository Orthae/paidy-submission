use crate::domain::item::Item;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ItemRepository {
    async fn find_item(&self, table_id: &i64, item_id: &Uuid) -> Result<Option<Item>, RepositoryError>;
    async fn find_items_by_table(&self, table_id: &i64) -> Result<Vec<Item>, RepositoryError>;
    async fn save_items(&self, item: &Vec<Item>) -> Result<(), RepositoryError>;
    async fn delete_item(&self, item_id: &Uuid) -> Result<(), RepositoryError>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RepositoryError {
    InternalRepositoryError(String),
    MappingError(String),
}
