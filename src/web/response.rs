use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::application::item_service::ItemModel;

#[derive(Default, Serialize)]
pub struct ItemDetailsResponse {
    pub id: Uuid,
    pub table_id: i64,
    pub name: String,
    pub preparation_time: DateTime<Utc>
}

#[derive(Default, Serialize)]
pub struct CreateItemsResponse {
    pub items: Vec<ItemModel>
}

impl From<Vec<ItemModel>> for CreateItemsResponse {
    fn from(items: Vec<ItemModel>) -> Self {
        CreateItemsResponse { items }
    }
}

#[derive(Default, Serialize)]
pub struct ListItemsResponse {
    pub items: Vec<ItemModel>
}

impl From<Vec<ItemModel>> for ListItemsResponse {
    fn from(items: Vec<ItemModel>) -> Self {
        ListItemsResponse { items }
    }
}

#[derive(Default, Serialize)]
pub struct ItemSummary {
    pub id: String,
    pub name: String,
}