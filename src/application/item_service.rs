use crate::domain::item::{Item, ItemValidationError};
use crate::domain::item_factory::ItemFactory;
use crate::domain::repository::{ItemRepository, RepositoryError};
use chrono::{DateTime, Utc};

use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ItemService {
    async fn create_items(&self, table_id: i64, command: CreateItemsCommand) -> Result<Vec<ItemModel>, ApplicationError>;
    async fn get_item(&self, table_id: i64, item_id: Uuid) -> Result<ItemModel, ApplicationError>;
    async fn get_items(&self, table_id: i64) -> Result<Vec<ItemModel>, ApplicationError>;
    async fn delete_item(&self, table_id: i64, item_id: Uuid) -> Result<(), ApplicationError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicationError {
    InternalError,
    ValidationError(String),
    ResourceNotFound,
}

pub struct ItemServiceImpl {
    repository: Arc<dyn ItemRepository + Send + Sync>,
    factory: Arc<dyn ItemFactory + Send + Sync>,
}

impl ItemServiceImpl {
    pub fn new(
        repository: Arc<dyn ItemRepository + Send + Sync>,
        factory: Arc<dyn ItemFactory + Send + Sync>
    ) -> Self {
        Self { repository, factory }
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
        
        if command.items.is_empty() {
            return Err(ApplicationError::ValidationError("Items list is empty.".to_string()));
        }

        let items = command
            .items
            .into_iter()
            .map(|model| self.factory.try_create(table_id, model.name))
            .collect::<Result<Vec<Item>, ItemValidationError>>()?;

        self.repository.save_items(&items).await?;
        
        let models = items
            .into_iter()
            .map(|item| ItemModel::from(item))
            .collect();
        
        Ok(models)
    }

    async fn get_item(&self, table_id: i64, item_id: Uuid) -> Result<ItemModel, ApplicationError> {
        info!("Getting item with id: {:?} for table: {:?}", item_id, table_id);
    
        let item = self
            .repository
            .find_item(&table_id, &item_id)
            .await?
            .map(|item| ItemModel::from(item))
            .ok_or(ApplicationError::ResourceNotFound)?;
    
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
    
    async fn delete_item(&self, table_id: i64, item_id: Uuid) -> Result<(), ApplicationError> {
        info!("Deleting item with id: {:?} for table: {:?}", item_id, table_id);
    
        self.repository.delete_item(&table_id, &item_id).await?;
    
        Ok(())
    }
}

impl From<RepositoryError> for ApplicationError {
    fn from(_: RepositoryError) -> Self {
        ApplicationError::InternalError
    }
}

impl From<ItemValidationError> for ApplicationError {
    fn from(error: ItemValidationError) -> Self {
        ApplicationError::ValidationError(error.to_string())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CreateItemsCommand {
    pub items: Vec<CreateItemModel>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CreateItemModel {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize)]
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
