use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{Field, Meta},
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct TopicIds {
    pub openalex: String,
    pub wikipedia: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Topic {
    pub description: String,
    pub display_name: String,
    pub domain: Field,
    pub field: Field,
    pub id: String,
    pub ids: TopicIds,
    pub keywords: Vec<String>,
    pub subfield: Field,
    pub updated_date: String,
    pub works_count: u32,

    pub cited_by_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TopicResponse {
    pub meta: Meta,
    pub results: Vec<Topic>,
}

impl_try_from_for_single_entity!(Topic);
impl_try_from_for_entity_response!(TopicResponse);

impl APIEntity<Topic, TopicResponse> for Topic {
    const API_URL: &'static str = "https://api.openalex.org/topics";
}
