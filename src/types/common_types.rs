use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub id: String,
    pub display_name: String,
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
    country_code: String,
    display_name: String,
    id: String,
    lineage: Vec<String>,
    ror: String,
    #[serde(rename = "type")]
    institution_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedAuthor {
    pub id: String,
    pub display_name: String,
    pub orcid: Option<String>,
}
