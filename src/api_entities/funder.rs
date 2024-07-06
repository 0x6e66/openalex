use serde_derive::{Deserialize, Serialize};

use crate::{
    api_entities::{
        common_types::{CountByYear, Meta, Role, SummaryStats},
        APIEntity,
    },
    impl_try_from_for_entity_response, impl_try_from_for_single_entity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct FunderIds {
    pub openalex: String,
    pub ror: Option<String>,
    pub wikidata: Option<String>,
    pub crossref: String,
    pub doi: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Funder {
    pub alternate_titles: Vec<String>,
    pub cited_by_count: u32,
    pub country_code: Option<String>,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub description: Option<String>,
    pub display_name: String,
    pub grants_count: u32,
    pub homepage_url: Option<String>,
    pub id: String,
    pub ids: FunderIds,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub roles: Vec<Role>,
    pub summary_stats: SummaryStats,
    pub updated_date: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FunderResponse {
    pub meta: Meta,
    pub results: Vec<Funder>,
}

impl_try_from_for_single_entity!(Funder);
impl_try_from_for_entity_response!(FunderResponse);

impl APIEntity<Funder, FunderResponse> for Funder {
    const API_URL: &'static str = "https://api.openalex.org/funders";
}
