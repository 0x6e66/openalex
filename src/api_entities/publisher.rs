use serde_derive::{Deserialize, Serialize};

use crate::{
    api_entities::{
        common_types::{CountByYear, Field, Meta, Role, SummaryStats},
        APIEntity,
    },
    impl_try_from_for_entity_response, impl_try_from_for_single_entity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct PublisherIds {
    pub openalex: String,
    pub ror: Option<String>,
    pub wikidata: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Publisher {
    pub alternate_titles: Vec<String>,
    pub cited_by_count: u32,
    pub country_codes: Vec<String>,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub display_name: String,
    pub hierarchy_level: u32,
    pub id: String,
    pub ids: PublisherIds,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub lineage: Vec<String>,
    pub parent_publisher: Option<Field>,
    pub roles: Vec<Role>,
    pub sources_api_url: String,
    pub summary_stats: SummaryStats,
    pub updated_date: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublisherResponse {
    pub meta: Meta,
    pub results: Vec<Publisher>,
}

impl_try_from_for_single_entity!(Publisher);
impl_try_from_for_entity_response!(PublisherResponse);

impl APIEntity<Publisher, PublisherResponse> for Publisher {
    const API_URL: &'static str = "https://api.openalex.org/publishers";
}
