use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{DehydratedAuthor, DehydratedInstitution, Field, Meta},
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct WorkIds {
    pub openalex: String,
    pub doi: Option<String>,
    pub mag: Option<String>,
    pub pmid: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorkSource {
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
    pub source: Option<WorkSource>,
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
pub struct Authorship {
    pub author_position: String,
    pub author: DehydratedAuthor,
    pub institutions: Vec<DehydratedInstitution>,
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
pub struct WorkTopic {
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
pub struct Work {
    pub id: String,
    pub doi: Option<String>,
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub publication_year: u32,
    // TODO: Maybe use chrono::datetime
    // ISO 8601 formatted
    pub publication_date: String,
    pub ids: WorkIds,
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
    pub primary_topic: Option<WorkTopic>,
    pub topics: Vec<WorkTopic>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct WorkResponse {
    pub meta: Meta,
    pub results: Vec<Work>,
}

impl_try_from_for_single_entity!(Work);
impl_try_from_for_entity_response!(WorkResponse);

impl APIEntity<Work, WorkResponse> for Work {
    const API_URL: &'static str = "https://api.openalex.org/works";
}
