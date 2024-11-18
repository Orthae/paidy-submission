use std::time::Duration;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Item {
    pub id: Uuid,
    pub table_id: String,
    pub name: String,
    pub preparation_time: DateTime<Utc>,
}

impl Item {
    pub fn new(table_id: String, name: String) -> Self {
        let offset: u64 = (rand::random::<u64>() % 600) + 300;

        Item {
            id: Uuid::now_v7(),
            table_id,
            name,
            preparation_time: Utc::now() + Duration::from_secs(offset)
        }
    }
}