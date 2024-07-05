use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{
        CountByYear, DehydratedInstitution, DehydratedTopic, DehydratedTopicShare, Meta, Role,
        SummaryStats,
    },
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AssociatedInstitution {
    #[serde(flatten)]
    pub institution: DehydratedInstitution,
    pub relationship: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Geo {
    pub city: String,
    pub geonames_city_id: Option<String>,
    pub region: Option<String>,
    pub country_code: Option<String>,
    pub country: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstitutionIds {
    pub openalex: String,
    pub ror: String,
    pub grid: Option<String>,
    pub wikipedia: Option<String>,
    pub wikidata: Option<String>,
    pub mag: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct International {
    pub display_name: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    pub id: String,
    pub display_name: String,
    pub host_organization: String,
    pub host_organization_name: String,
    pub host_organization_lineage: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Institution {
    pub associated_institutions: Vec<AssociatedInstitution>,
    pub cited_by_count: u32,
    pub country_code: Option<String>,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub display_name: String,
    pub display_name_acronyms: Vec<String>,
    pub display_name_alternatives: Vec<String>,
    pub geo: Geo,
    pub homepage_url: Option<String>,
    pub id: String,
    pub ids: InstitutionIds,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub international: International,
    pub lineage: Vec<String>,
    pub repositories: Vec<Repository>,
    pub roles: Vec<Role>,
    pub ror: String,
    pub summary_stats: SummaryStats,
    #[serde(rename = "type")]
    pub institution_type: String,
    pub updated_date: String,
    pub works_api_url: String,
    pub works_count: u32,

    pub topics: Vec<DehydratedTopic>,
    pub topic_share: Vec<DehydratedTopicShare>,
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
