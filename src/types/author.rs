use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{
    common_types::{CountByYear, DehydratedInstitution, DehydratedTopic, DehydratedTopicShare, Meta, SummaryStats},
    APIEntity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorIds {
    pub openalex: String,
    pub orcid: Option<String>,
    pub scopus: Option<String>,
    pub twitter: Option<String>,
    pub wikipedia: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Affiliation {
    pub institution: DehydratedInstitution,
    pub years: Vec<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub affiliations: Vec<Affiliation>,
    pub cited_by_count: u32,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub display_name: String,
    pub display_name_alternatives: Vec<String>,
    pub id: String,
    pub ids: AuthorIds,
    pub last_known_institutions: Vec<DehydratedInstitution>,
    pub orcid: Option<String>,
    pub summary_stats: SummaryStats,
    pub topics: Vec<DehydratedTopic>,
    pub topic_share: Vec<DehydratedTopicShare>,
    pub updated_date: String,
    pub works_api_url: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorResponse {
    pub meta: Meta,
    pub results: Vec<Author>,
}

impl_try_from_for_single_entity!(Author);
impl_try_from_for_entity_response!(AuthorResponse);

impl APIEntity<Author, AuthorResponse> for Author {
    const API_URL: &'static str = "https://api.openalex.org/authors";
}
