use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountByYear {
    pub year: u32,
    pub works_count: u32,
    pub cited_by_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    pub _2yr_mean_citedness: f32,
    pub h_index: u32,
    pub i10_index: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Role {
    pub role: String,
    pub id: String,
    pub works_count: u32,
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
pub struct DehydratedInstitution {
    pub country_code: String,
    pub display_name: String,
    pub id: String,
    pub lineage: Option<Vec<String>>,
    pub ror: String,
    #[serde(rename = "type")]
    pub institution_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedAuthor {
    pub id: String,
    pub display_name: String,
    pub orcid: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedConcept {
    pub display_name: String,
    pub id: String,
    pub level: u32,
    pub wikidata: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct International {
    pub display_name: HashMap<String, String>,
}

// not from OA, self created
#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedTopic {
    pub id: String,
    pub display_name: String,
    pub count: u32,
    pub subfield: Field,
    pub field: Field,
    pub domain: Field,
}

// not from OA, self created
#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedTopicShare {
    pub id: String,
    pub display_name: String,
    pub value: f64,
    pub subfield: Field,
    pub field: Field,
    pub domain: Field,
}
