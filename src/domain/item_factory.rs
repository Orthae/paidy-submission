use crate::domain::item::{Item, ItemValidationError};
use chrono::{DateTime, SubsecRound, Utc};
use rand::Rng;
use std::time::Duration;
use uuid::Uuid;

pub struct ItemFactory;

impl ItemFactory {
    pub fn try_create(table_id: i64, name: String) -> Result<Item, ItemValidationError> {
        let id = Uuid::now_v7();
        let preparation_time = Self::get_random_preparation_time();
        Item::try_new(id, table_id, name, preparation_time)
    }

    fn get_random_preparation_time() -> DateTime<Utc> {
        let offset = rand::thread_rng().gen_range(300..901);
        Utc::now().round_subsecs(6) + Duration::from_secs(offset)
    }
}
