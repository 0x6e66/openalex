use serde_derive::{Deserialize, Serialize};

use crate::{
    api_entities::{common_types::Meta, APIEntity},
    impl_try_from_for_entity_response, impl_try_from_for_single_entity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Keyword {
    pub cited_by_count: u32,
    pub created_date: String,
    pub display_name: String,
    pub id: String,
    pub updated_date: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KeywordResponse {
    pub meta: Meta,
    pub results: Vec<Keyword>,
}

impl_try_from_for_single_entity!(Keyword);
impl_try_from_for_entity_response!(KeywordResponse);

impl APIEntity<Keyword, KeywordResponse> for Keyword {
    const API_URL: &'static str = "https://api.openalex.org/keywords";
}
