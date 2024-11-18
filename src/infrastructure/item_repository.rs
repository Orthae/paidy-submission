use anyhow::Result;
use uuid::Uuid;
use crate::domain::item::Item;
use crate::domain::repository::ItemRepository;

struct ItemRepositoryImpl;

impl ItemRepository for ItemRepositoryImpl {
    async fn find_item(&self, item_id: Uuid) -> Result<Option<Item>> {
        todo!()
    }

    async fn find_items_by_table(&self, table_id: String) -> Result<Vec<Item>> {
        todo!()
    }

    async fn save_items(&self, item: Vec<Item>) -> Result<()> {
        todo!()
    }

    async fn delete_item(&self, item_id: Uuid) -> Result<()> {
        todo!()
    }
}

enum RepositoryError {
    InternalError
}

