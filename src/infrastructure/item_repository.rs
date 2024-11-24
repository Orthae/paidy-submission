use crate::domain::item::{Item, ItemValidationError};
use crate::domain::repository::{ItemRepository, RepositoryError};
use async_trait::async_trait;

use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use tracing::error;
use uuid::Uuid;

const QUERY_ITEM: &str =
    "SELECT id, table_id, name, preparation_time FROM items WHERE id = $2 and table_id = $1";
const QUERY_TABLE: &str =
    "SELECT id, table_id, name, preparation_time FROM items WHERE table_id = $1";
const INSERT_ITEM: &str =
    "INSERT INTO items (id, table_id, name, preparation_time) VALUES ($1, $2, $3, $4)";
const DELETE_ITEM: &str = "DELETE FROM items WHERE id = $2 AND table_id = $1";

#[derive(Clone)]
pub struct ItemRepositoryImpl {
    pool: Pool<Postgres>,
}

impl ItemRepositoryImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        ItemRepositoryImpl { pool }
    }
}

#[async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn find_item(
        &self,
        table_id: &i64,
        item_id: &Uuid,
    ) -> Result<Option<Item>, RepositoryError> {
        sqlx::query(QUERY_ITEM)
            .bind(table_id)
            .bind(item_id)
            .fetch_optional(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query item. Error: {:?}", e))?
            .map(Item::try_from)
            .transpose()
    }

    async fn find_items_by_table(&self, table_id: &i64) -> Result<Vec<Item>, RepositoryError> {
        sqlx::query(QUERY_TABLE)
            .bind(table_id)
            .fetch_all(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query table. Error: {:?}", e))?
            .into_iter()
            .map(Item::try_from)
            .collect()
    }

    async fn save_items(&self, items: &[Item]) -> Result<(), RepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .inspect_err(|e| error!("Failed to begin transaction. Error: {:?}", e))?;

        for entity in items {
            sqlx::query(INSERT_ITEM)
                .bind(entity.id)
                .bind(entity.table_id)
                .bind(&entity.name)
                .bind(entity.preparation_time)
                .execute(&mut *transaction)
                .await
                .inspect_err(|e| error!("Inserting item failed. Error: {:?}", e))?;
        }

        transaction
            .commit()
            .await
            .inspect_err(|e| error!("Failed to commit transaction. Error: {:?}", e))?;

        Ok(())
    }

    async fn delete_item(&self, table_id: &i64, item_id: &Uuid) -> Result<(), RepositoryError> {
        sqlx::query(DELETE_ITEM)
            .bind(table_id)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to delete item. Error: {:?}", e))?;

        Ok(())
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        RepositoryError::InternalRepositoryError(error.to_string())
    }
}

impl From<ItemValidationError> for RepositoryError {
    fn from(error: ItemValidationError) -> Self {
        RepositoryError::MappingError(error.to_string())
    }
}

impl TryFrom<PgRow> for Item {
    type Error = RepositoryError;

    fn try_from(row: PgRow) -> Result<Self, RepositoryError> {
        let id: Uuid = row.try_get(0)?;
        let table_id: i64 = row.try_get(1)?;
        let name: String = row.try_get(2)?;
        let preparation_time = row.try_get(3)?;

        let item = Item::try_new(id, table_id, name, preparation_time)
            .inspect_err(|e| error!("Failed to create item. Error: {:?}", e))
            .map_err(|e| RepositoryError::MappingError(e.to_string()))?;

        Ok(item)
    }
}
