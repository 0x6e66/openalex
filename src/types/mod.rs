pub mod work;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub count: u32,
    pub db_response_time_ms: u32,
    pub page: u32,
    pub per_page: u32,
    pub groups_count: Option<u32>,
}

