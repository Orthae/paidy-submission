use anyhow::Result;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;
use log::error;
use sqlx::postgres::PgRow;
use thiserror::Error;
use crate::domain::item::Item;
use crate::domain::repository::ItemRepository;

const QUERY_ITEM: &str = "SELECT id, table_id, name, preparation_time FROM items WHERE id = $1";
const QUERY_TABLE: &str = "SELECT id, table_id, name, preparation_time FROM items WHERE table_id = $1";

struct ItemRepositoryImpl {
    pool: Pool<Postgres>
}

impl ItemRepository for ItemRepositoryImpl {
    async fn find_item(&self, item_id: Uuid) -> Result<Option<Item>> {
        sqlx::query(QUERY_ITEM)
            .bind(item_id)
            .fetch_optional(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query item. Error: {:?}", e))?
            .map(|row| Item::try_from(row))
            .transpose()
    }

    async fn find_items_by_table(&self, table_id: String) -> Result<Vec<Item>> {
        sqlx::query(QUERY_TABLE)
            .bind(table_id)
            .fetch_all(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query table. Error: {:?}", e))?
            .into_iter()
            .map(|row| Item::try_from(row))
            .collect()
    }

    async fn save_items(&self, item: Vec<Item>) -> Result<()> {
        todo!()
    }

    async fn delete_item(&self, item_id: Uuid) -> Result<()> {
        todo!()
    }
}

#[derive(Debug, Error)]
enum PostgresRepositoryError {
    #[error("Internal repository error: {0}")]
    InternalError(sqlx::Error),
}

impl From<sqlx::Error> for PostgresRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        PostgresRepositoryError::InternalError(error)
    }
}

impl TryFrom<PgRow> for Item {
    type Error = anyhow::Error;

    fn try_from(row: PgRow) -> std::result::Result<Self, Self::Error> {
        let item: Item = Item {
            id: row.try_get(0)?,
            table_id: row.try_get(1)?,
            name: row.try_get(2)?,
            preparation_time: row.try_get(3)?,
        };

        Ok(item)
    }
}

