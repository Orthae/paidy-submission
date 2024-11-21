use crate::domain::item::{Item, ItemValidationError};
use crate::domain::item_factory::ItemFactory;
use crate::domain::repository::{ItemRepository, RepositoryError};
use chrono::{DateTime, Utc};
use log::info;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ItemService {
    async fn create_items(&self, table_id: i64, command: CreateItemsCommand) -> Result<Vec<ItemModel>, ApplicationError>;
    async fn get_item(&self, id: Uuid) -> Result<Option<ItemModel>, ApplicationError>;
    async fn get_items(&self, table_id: i64) -> Result<Vec<ItemModel>, ApplicationError>;
    async fn delete_item(&self, id: Uuid) -> Result<(), ApplicationError>;
}

pub struct ItemServiceImpl {
    repository: Arc<dyn ItemRepository + Send + Sync>,
}

impl ItemServiceImpl {
    pub fn new(repository: Arc<dyn ItemRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ItemService for ItemServiceImpl {
    async fn create_items(
        &self,
        table_id: i64,
        command: CreateItemsCommand,
    ) -> Result<Vec<ItemModel>, ApplicationError> {
        info!("Creating items from command: {:?}", command);

        let items = command
            .items
            .into_iter()
            .map(|model| ItemFactory::try_create(table_id, model.name))
            .collect::<Result<Vec<Item>, ItemValidationError>>()?;

        self.repository.save_items(&items).await?;
        
        let models = items
            .into_iter()
            .map(|item| ItemModel::from(item))
            .collect();
        
        Ok(models)
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<ItemModel>, ApplicationError> {
        info!("Getting item with id: {:?}", id);
    
        let item = self
            .repository
            .find_item(&id)
            .await?
            .map(|item| ItemModel::from(item));
    
        Ok(item)
    }
    
    async fn get_items(&self, table_id: i64) -> Result<Vec<ItemModel>, ApplicationError> {
        info!("Getting items for table: {:?}", table_id);
    
        let models = self
            .repository
            .find_items_by_table(&table_id)
            .await?
            .into_iter()
            .map(|item| ItemModel::from(item))
            .collect();
    
        Ok(models)
    }
    
    async fn delete_item(&self, id: Uuid) -> Result<(), ApplicationError> {
        info!("Deleting item with id: {:?}", id);
    
        self.repository.delete_item(&id).await?;
    
        Ok(())
    }
}

#[derive(Debug)]
pub enum ApplicationError {
    RepositoryError(RepositoryError),
    ItemValidationError(ItemValidationError),
}

impl From<RepositoryError> for ApplicationError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<ItemValidationError> for ApplicationError {
    fn from(value: ItemValidationError) -> Self {
        Self::ItemValidationError(value)
    }
}

#[derive(Debug)]
pub struct CreateItemsCommand {
    pub items: Vec<CreateItemModel>,
}

#[derive(Debug)]
pub struct CreateItemModel {
    pub name: String,
}

pub struct ItemModel {
    pub id: Uuid,
    pub table_id: i64,
    pub name: String,
    pub preparation_time: DateTime<Utc>,
}

impl From<Item> for ItemModel {
    fn from(value: Item) -> Self {
        Self {
            id: value.id,
            table_id: value.table_id,
            name: value.name,
            preparation_time: value.preparation_time,
        }
    }
}
