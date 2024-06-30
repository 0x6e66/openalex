use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::prelude::*;

const API_URL: &str = "https://api.openalex.org/works";

#[derive(Deserialize, Serialize, Debug)]
pub struct Ids {
    pub openalex: String,
    pub doi: Option<String>,
    pub mag: Option<String>,
    pub pmid: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub id: String,
    pub display_name: String,
    pub issn_l: Option<String>,
    pub issn: Option<Vec<String>>,
    pub host_organization: Option<String>,
    #[serde(rename = "type")]
    pub source_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Location {
    pub is_oa: bool,
    pub landing_page_url: Option<String>,
    pub pdf_url: Option<String>,
    pub source: Option<Source>,
    pub license: Option<String>,
    pub license_id: Option<String>,
    pub version: Option<String>,
    // pub is_accepted: bool,
    // pub is_published: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenAccess {
    pub is_oa: bool,
    pub oa_status: String,
    pub oa_url: String,
    pub any_repository_has_fulltext: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub id: String,
    pub display_name: String,
    pub orcid: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Institution {
    pub id: String,
    pub display_name: String,
    pub ror: String,
    pub country_code: String,
    #[serde(rename = "type")]
    pub instituion_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Authorship {
    pub author_position: String,
    pub author: Author,
    pub institutions: Vec<Institution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct APC {
    pub value: u32,
    pub currency: String,
    pub value_usd: Option<u32>,
    pub provenance: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Biblio {
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Topic {
    pub id: String,
    pub display_name: String,
    pub score: f32,
    pub subfield: Field,
    pub field: Field,
    pub domain: Field,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Keyword {
    pub id: String,
    pub display_name: String,
    pub score: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Concept {
    pub id: String,
    pub wikidata: String,
    pub display_name: String,
    pub level: u32,
    pub score: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YearCount {
    pub year: u32,
    pub cited_by_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MeshTag {
    pub descriptor_ui: String,
    pub descriptor_name: String,
    pub qualifier_ui: String,
    pub qualifier_name: Option<String>,
    pub is_major_topic: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Grant {
    pub funder: String,
    pub funder_display_name: String,
    pub award_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CitedByPercentileYear {
    pub min: u32,
    pub max: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub count: u32,
    pub db_response_time_ms: u32,
    pub page: u32,
    pub per_page: u32,
    pub groups_count: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub meta: Meta,
    pub results: Vec<Work>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Work {
    pub id: String,
    pub doi: Option<String>,
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub publication_year: u32,
    // TODO: Maybe use chrono::datetime
    // ISO 8601 formatted
    pub publication_date: String,
    pub ids: Ids,
    pub language: Option<String>,
    pub primary_location: Option<Location>,
    #[serde(rename = "type")]
    pub work_type: String,
    pub type_crossref: String,
    pub indexed_in: Vec<String>,
    pub authorships: Vec<Authorship>,
    pub countries_distinct_count: u32,
    pub institutions_distinct_count: u32,
    pub corresponding_author_ids: Vec<String>,
    pub corresponding_institution_ids: Vec<String>,
    pub apc_list: Option<APC>,
    pub apc_paid: Option<APC>,
    pub fwci: Option<f32>,
    pub has_fulltext: bool,
    pub cited_by_count: u32,
    pub cited_by_percentile_year: CitedByPercentileYear,
    pub biblio: Biblio,
    pub is_retracted: bool,
    pub is_paratext: bool,
    pub primary_topic: Option<Topic>,
    pub topics: Vec<Topic>,
    pub keywords: Vec<Keyword>,
    pub concepts: Vec<Concept>,
    pub mesh: Vec<MeshTag>,
    pub locations_count: u32,
    pub locations: Vec<Location>,
    pub best_oa_location: Option<Location>,
    pub sustainable_development_goals: Vec<Keyword>,
    pub grants: Vec<Grant>,
    // datasets ?
    // versions ?
    pub referenced_works_count: u32,
    pub referenced_works: Vec<String>,
    pub related_works: Vec<String>,
    pub ngrams_url: String,
    pub abstract_inverted_index: Option<HashMap<String, Vec<u32>>>,
    pub cited_by_api_url: String,
    pub counts_by_year: Vec<YearCount>,
    pub updated_date: String,
    pub created_date: String,
}

impl TryFrom<reqwest::blocking::Response> for Work {
    type Error = Error;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self> {
        if !response.status().is_success() {
            return Err(Error::OpenAlex(format!(
                "HTTP Response code was '{}'",
                response.status()
            )));
        }

        let work = response.json::<Self>()?;
        Ok(work)
    }
}

impl TryFrom<reqwest::blocking::Response> for ApiResponse {
    type Error = Error;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self> {
        if !response.status().is_success() {
            return Err(Error::OpenAlex(format!(
                "HTTP Response code was '{}'",
                response.status()
            )));
        }

        let response = response.json::<Self>()?;
        Ok(response)
    }
}

impl Work {
    pub fn new(id: &str) -> Result<Self> {
        let url = format!("{}/W{}", API_URL, id);
        let response = reqwest::blocking::get(url)?;
        response.try_into()
    }

    pub fn get_samples(number_of_samples: u32, seed: u32) -> Result<ApiResponse> {
        let client = Client::new();
        let response = client
            .get(API_URL)
            .query(&[("sample", number_of_samples), ("seed", seed)])
            .send()?;
        response.try_into()
    }
}
