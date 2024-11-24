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

        Ok(Item {
            id,
            table_id,
            name,
            preparation_time,
        })
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ItemValidationError {
    #[error("Name cannot be empty.")]
    EmptyName,
    #[error("Table id cannot be negative.")]
    NegativeTableId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_item() {
        let id = Uuid::new_v4();
        let table_id = 1;
        let name = "Pierogi".to_string();
        let preparation_time = Utc::now();

        let item = Item::try_new(id, table_id, name.clone(), preparation_time)
            .expect("Failed to create item");

        assert_eq!(item.id, id);
        assert_eq!(item.table_id, table_id);
        assert_eq!(item.name, name);
        assert_eq!(item.preparation_time, preparation_time);
    }

    #[test]
    fn should_not_create_item_with_empty_name() {
        let id = Uuid::new_v4();
        let table_id = 1;
        let name = "".to_string();
        let preparation_time = Utc::now();

        let result = Item::try_new(id, table_id, name, preparation_time);

        assert_eq!(result, Err(ItemValidationError::EmptyName));
    }

    #[test]
    fn should_not_create_item_with_negative_table_id() {
        let id = Uuid::new_v4();
        let table_id = -1;
        let name = "Pierogi".to_string();
        let preparation_time = Utc::now();

        let result = Item::try_new(id, table_id, name, preparation_time);

        assert_eq!(result, Err(ItemValidationError::NegativeTableId));
    }
}
