use serde_derive::{Deserialize, Serialize};

use crate::{
    api_entities::{
        common_types::{CountByYear, Meta, SummaryStats},
        APIEntity,
    },
    impl_try_from_for_entity_response, impl_try_from_for_single_entity,
    utils::{deserialize_null_default, deserialize_opt_int_to_opt_string},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct APCPrice {
    pub price: u32,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceIds {
    pub fatcat: Option<String>,
    #[serde(default)]
    pub issn: Vec<String>,
    pub issn_l: Option<String>,
    #[serde(deserialize_with = "deserialize_opt_int_to_opt_string")]
    pub mag: Option<String>,
    pub openalex: String,
    pub wikidata: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Society {
    pub url: String,
    pub organization: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub abbreviated_title: Option<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub alternate_titles: Vec<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub apc_prices: Vec<APCPrice>,
    pub apc_usd: Option<u32>,
    pub cited_by_count: u32,
    pub country_code: Option<String>,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub display_name: String,
    pub homepage_url: Option<String>,
    pub host_organization: Option<String>,
    pub host_organization_lineage: Vec<String>,
    pub host_organization_name: Option<String>,
    pub id: String,
    pub ids: SourceIds,
    pub is_in_doaj: bool,
    pub is_oa: bool,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub issn: Vec<String>,
    pub issn_l: Option<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub societies: Vec<Society>,
    pub summary_stats: SummaryStats,
    #[serde(rename = "type")]
    pub source_type: String,
    pub updated_date: String,
    pub works_api_url: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceResponse {
    pub meta: Meta,
    pub results: Vec<Source>,
}

impl_try_from_for_single_entity!(Source);
impl_try_from_for_entity_response!(SourceResponse);

impl APIEntity<Source, SourceResponse> for Source {
    const API_URL: &'static str = "https://api.openalex.org/sources";
}
