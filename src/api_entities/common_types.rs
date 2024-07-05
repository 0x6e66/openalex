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
    country_code: String,
    display_name: String,
    id: String,
    lineage: Option<Vec<String>>,
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

// not from OA, self created
#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedTopic {
    id: String,
    display_name: String,
    count: u32,
    subfield: Field,
    field: Field,
    domain: Field,
}

// not from OA, self created
#[derive(Deserialize, Serialize, Debug)]
pub struct DehydratedTopicShare {
    id: String,
    display_name: String,
    value: f64,
    subfield: Field,
    field: Field,
    domain: Field,
}
