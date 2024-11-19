use anyhow::Result;
use uuid::Uuid;
use crate::domain::item::Item;

pub trait ItemRepository {
    async fn find_item(&self, item_id: &Uuid) -> Result<Option<Item>>;
    async fn find_items_by_table(&self, table_id: &String) -> Result<Vec<Item>>;
    async fn save_items(&self, item: &Vec<Item>) -> Result<()>;
    async fn delete_item(&self, item_id: &Uuid) -> Result<()>;
}