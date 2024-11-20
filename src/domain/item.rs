use chrono::{DateTime, SubsecRound, Utc};
use rand::Rng;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub id: Uuid,
    pub table_id: String,
    pub name: String,
    pub preparation_time: DateTime<Utc>,
}

impl Item {
    pub fn new(table_id: String, name: String) -> Self {
        Item {
            id: Uuid::now_v7(),
            table_id,
            name,
            preparation_time: Item::get_random_preparation_time(),
        }
    }

    fn get_random_preparation_time() -> DateTime<Utc> {
        let offset = rand::thread_rng().gen_range(300..901);
        Utc::now().round_subsecs(6) + Duration::from_secs(offset)
    }
}
