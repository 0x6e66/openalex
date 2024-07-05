use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{common_types::Meta, APIEntity};

#[derive(Deserialize, Serialize, Debug)]
pub struct Keyword {
    cited_by_count: u32,
    created_date: String,
    display_name: String,
    id: String,
    updated_date: String,
    works_count: u32,
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
