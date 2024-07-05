use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{CountByYear, Field, Meta, Role, SummaryStats},
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct PublisherIds {
    openalex: String,
    ror: Option<String>,
    wikidata: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Publisher {
    alternate_titles: Vec<String>,
    cited_by_count: u32,
    country_codes: Vec<String>,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    hierarchy_level: u32,
    id: String,
    ids: PublisherIds,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    lineage: Vec<String>,
    parent_publisher: Option<Field>,
    roles: Vec<Role>,
    sources_api_url: String,
    summary_stats: SummaryStats,
    updated_date: String,
    works_count: u32,
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
