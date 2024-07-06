use serde_derive::{Deserialize, Serialize};

use crate::{impl_try_from_for_entity_response, impl_try_from_for_single_entity};

use super::{common_types::{CountByYear, DehydratedConcept, International, Meta, SummaryStats}, APIEntity};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConceptIds {
    pub mag: String,
    pub openalex: String,
    pub umls_cui: Option<Vec<String>>,
    pub umls_aui: Option<Vec<String>>,
    pub wikidata: String,
    pub wikipedia: Option<String>,
}

/// Note from OpenAlex: https://docs.openalex.org/api-entities/concepts
#[derive(Deserialize, Serialize, Debug)]
pub struct Concept {
    ancestors: Vec<DehydratedConcept>,
    cited_by_count: u32,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    description: Option<String>,
    display_name: String,
    id: String,
    ids: ConceptIds,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    international: International,
    level: u32,
    related_concepts: Vec<DehydratedConcept>,
    summary_stats: SummaryStats,
    updated_date: String,
    wikidata: String,
    works_api_url: String,
    works_count: u32,
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