use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub id: Uuid,
    pub table_id: i64,
    pub name: String,
    pub preparation_time: DateTime<Utc>,
}

impl Item {
    pub fn try_new(
        id: Uuid,
        table_id: i64,
        name: String,
        preparation_time: DateTime<Utc>,
    ) -> Result<Self, ItemValidationError> {
        if name.is_empty() {
            return Err(ItemValidationError::EmptyName);
        }

        if table_id.is_negative() {
            return Err(ItemValidationError::NegativeTableId);
        }

        Ok(Item { id, table_id, name, preparation_time })
    }
}

#[derive(Debug, Error)]
pub enum ItemValidationError {
    #[error("Name cannot be empty.")]
    EmptyName,
    #[error("Table id cannot be negative.")]
    NegativeTableId,
}
