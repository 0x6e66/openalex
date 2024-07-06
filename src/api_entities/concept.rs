use serde_derive::{Deserialize, Serialize};

use crate::{
    api_entities::{
        common_types::{CountByYear, DehydratedConcept, International, Meta, SummaryStats},
        APIEntity,
    },
    impl_try_from_for_entity_response, impl_try_from_for_single_entity,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConceptIds {
    pub mag: String,
    pub openalex: String,
    #[serde(default)]
    pub umls_cui: Vec<String>,
    #[serde(default)]
    pub umls_aui: Vec<String>,
    pub wikidata: String,
    pub wikipedia: Option<String>,
}

/// Note from OpenAlex: https://docs.openalex.org/api-entities/concepts
#[derive(Deserialize, Serialize, Debug)]
pub struct Concept {
    pub ancestors: Vec<DehydratedConcept>,
    pub cited_by_count: u32,
    pub counts_by_year: Vec<CountByYear>,
    pub created_date: String,
    pub description: Option<String>,
    pub display_name: String,
    pub id: String,
    pub ids: ConceptIds,
    pub image_thumbnail_url: Option<String>,
    pub image_url: Option<String>,
    pub international: International,
    pub level: u32,
    pub related_concepts: Vec<DehydratedConcept>,
    pub summary_stats: SummaryStats,
    pub updated_date: String,
    pub wikidata: String,
    pub works_api_url: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConceptResponse {
    pub meta: Meta,
    pub results: Vec<Concept>,
}

impl_try_from_for_single_entity!(Concept);
impl_try_from_for_entity_response!(ConceptResponse);

impl APIEntity<Concept, ConceptResponse> for Concept {
    const API_URL: &'static str = "https://api.openalex.org/concepts";
}
