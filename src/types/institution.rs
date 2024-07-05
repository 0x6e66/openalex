use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{
        CountByYear, DehydratedInstitution, DehydratedTopic, DehydratedTopicShare, Meta,
        SummaryStats,
    },
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AssociatedInstitution {
    #[serde(flatten)]
    institution: DehydratedInstitution,
    relationship: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Geo {
    city: String,
    geonames_city_id: Option<String>,
    region: Option<String>,
    country_code: Option<String>,
    country: String,
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstitutionIds {
    openalex: String,
    ror: String,
    grid: Option<String>,
    wikipedia: Option<String>,
    wikidata: Option<String>,
    mag: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct International {
    display_name: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    id: String,
    display_name: String,
    host_organization: String,
    host_organization_name: String,
    host_organization_lineage: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Role {
    role: String,
    id: String,
    works_count: u32,
}

pub struct InstitutionTopic {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Institution {
    associated_institutions: Vec<AssociatedInstitution>,
    cited_by_count: u32,
    country_code: Option<String>,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    display_name_acronyms: Vec<String>,
    display_name_alternatives: Vec<String>,
    geo: Geo,
    homepage_url: Option<String>,
    id: String,
    ids: InstitutionIds,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    international: International,
    lineage: Vec<String>,
    repositories: Vec<Repository>,
    roles: Vec<Role>,
    ror: String,
    summary_stats: SummaryStats,
    #[serde(rename = "type")]
    institution_type: String,
    updated_date: String,
    works_api_url: String,
    works_count: u32,

    topics: Vec<DehydratedTopic>,
    topic_share: Vec<DehydratedTopicShare>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstitutionResponse {
    pub meta: Meta,
    pub results: Vec<Institution>,
}

impl_try_from_for_single_entity!(Institution);
impl_try_from_for_entity_response!(InstitutionResponse);

impl APIEntity<Institution, InstitutionResponse> for Institution {
    const API_URL: &'static str = "https://api.openalex.org/institutions";
}
