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
const INSERT_ITEM: &str = "INSERT INTO items (id, table_id, name, preparation_time) VALUES ($1, $2, $3, $4)";
const DELETE_ITEM: &str = "DELETE FROM items WHERE id = $1";

pub struct ItemRepositoryImpl {
    pool: Pool<Postgres>
}

impl ItemRepositoryImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        ItemRepositoryImpl { pool }
    }
}

impl ItemRepository for ItemRepositoryImpl {
    async fn find_item(&self, item_id: &Uuid) -> Result<Option<Item>> {
        sqlx::query(QUERY_ITEM)
            .bind(item_id)
            .fetch_optional(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query item. Error: {:?}", e))?
            .map(|row| Item::try_from(row))
            .transpose()
    }

    async fn find_items_by_table(&self, table_id: &String) -> Result<Vec<Item>> {
        sqlx::query(QUERY_TABLE)
            .bind(table_id)
            .fetch_all(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to query table. Error: {:?}", e))?
            .into_iter()
            .map(|row| Item::try_from(row))
            .collect()
    }

    async fn save_items(&self, items: &Vec<Item>) -> Result<()> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .inspect_err(|e| error!("Failed to begin transaction. Error: {:?}", e))?;

        for entity in items {
            sqlx::query(INSERT_ITEM)
                .bind(&entity.id)
                .bind(&entity.table_id)
                .bind(&entity.name)
                .bind(&entity.preparation_time)
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

    async fn delete_item(&self, item_id: &Uuid) -> Result<()> {
        sqlx::query(DELETE_ITEM)
            .bind(item_id)
            .execute(&self.pool)
            .await
            .inspect_err(|e| error!("Failed to delete item. Error: {:?}", e))?;

        Ok(())
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

    fn try_from(row: PgRow) -> Result<Self> {
        let item: Item = Item {
            id: row.try_get(0)?,
            table_id: row.try_get(1)?,
            name: row.try_get(2)?,
            preparation_time: row.try_get(3)?,
        };

        Ok(item)
    }
}

