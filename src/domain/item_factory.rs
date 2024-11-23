use crate::domain::item::{Item, ItemValidationError};
use chrono::{DateTime, SubsecRound, Utc};
use rand::Rng;
use std::time::Duration;
use mockall::automock;
use uuid::Uuid;

#[automock]
pub trait ItemFactory {
    fn try_create(&self, table_id: i64, name: String) -> Result<Item, ItemValidationError>;
}

#[derive(Default, Debug)]
pub struct ItemFactoryImpl;

impl ItemFactoryImpl {
    fn get_random_preparation_time() -> DateTime<Utc> {
        let offset = rand::thread_rng().gen_range(300..901);
        Utc::now().round_subsecs(6) + Duration::from_secs(offset)
    }
}

impl ItemFactory for ItemFactoryImpl {
    fn try_create(&self, table_id: i64, name: String) -> Result<Item, ItemValidationError> {
        let id = Uuid::now_v7();
        let preparation_time = Self::get_random_preparation_time();
        Item::try_new(id, table_id, name, preparation_time)
    }
}
