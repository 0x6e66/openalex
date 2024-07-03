use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity, prelude::*};

use super::{
    common_types::{CountByYear, Meta, SummaryStats},
    filter::Filter,
    sort::Sort,
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct APCPrice {
    pub price: u32,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceIds {
    pub fatcat: String,
    pub issn: Vec<String>,
    pub issn_l: String,
    pub mag: u32,
    pub openalex: String,
    pub wikidata: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Society {
    url: String,
    organization: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub abbreviated_title: String,
    pub alternate_titles: Vec<String>,
    pub apc_prices: Vec<APCPrice>,
    pub apc_usd: u32,
    pub cited_by_count: u32,
    pub country_code: String,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub display_name: String,
    pub homepage_url: String,
    pub host_organization: String,
    pub host_organization_lineage: Vec<String>,
    pub host_organization_name: String,
    pub id: String,
    pub ids: SourceIds,
    pub is_in_doaj: bool,
    pub is_oa: bool,
    pub issn: Vec<String>,
    pub issn_l: String,
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

    fn new(id: &str) -> Result<Source> {
        let url = format!("{}/W{}", Self::API_URL, id);
        let response = reqwest::blocking::get(url)?;
        response.try_into()
    }

    fn get_samples(number_of_samples: u32, seed: impl Into<String>) -> Result<SourceResponse> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("sample", number_of_samples.to_string()),
                ("seed", seed.into()),
            ])
            .send()?;
        response.try_into()
    }

    fn filter(filter: Filter, page: u32, per_page: u32, sort: Sort) -> Result<SourceResponse> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("filter", filter.to_string()),
                ("page", page.to_string()),
                ("per-page", per_page.to_string()),
                ("sort", sort.to_string()),
            ])
            .send()?;

        response.try_into()
    }

    fn search(
        search: impl Into<String>,
        page: u32,
        per_page: u32,
        sort: Sort,
    ) -> Result<SourceResponse> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("search", search.into()),
                ("page", page.to_string()),
                ("per-page", per_page.to_string()),
                ("sort", sort.to_string()),
            ])
            .send()?;

        response.try_into()
    }
}
