use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::prelude::*;

use super::common_types::{DehydratedInstitution, Field, Meta};

const API_URL: &str = "https://api.openalex.org/authors";

#[derive(Deserialize, Serialize, Debug)]
pub struct SummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    _2yr_mean_citedness: f32,
    h_index: u32,
    i10_index: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorIds {
    openalex: String,
    orcid: Option<String>,
    scopus: Option<String>,
    twitter: Option<String>,
    wikipedia: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Affiliation {
    institution: DehydratedInstitution,
    years: Vec<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountByYear {
    year: u32,
    works_count: u32,
    cited_by_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorTopic {
    pub id: String,
    pub display_name: String,
    pub count: u32,
    pub subfield: Field,
    pub field: Field,
    pub domain: Field,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    affiliations: Vec<Affiliation>,
    cited_by_count: u32,
    counts_by_year: Vec<CountByYear>,
    // TODO: change for chrono::datatime
    created_date: String,
    display_name: String,
    display_name_alternatives: Vec<String>,
    id: String,
    ids: AuthorIds,
    last_known_institutions: Vec<DehydratedInstitution>,
    orcid: Option<String>,
    summary_stats: SummaryStats,
    // TODO: change for chrono::datatime
    topics: Vec<AuthorTopic>,
    updated_date: String,
    works_api_url: String,
    works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorResponse {
    meta: Meta,
    results: Vec<Author>,
}

impl TryFrom<reqwest::blocking::Response> for Author {
    type Error = Error;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self> {
        if !response.status().is_success() {
            return Err(Error::OpenAlex(format!(
                "HTTP Response code was '{}'",
                response.status()
            )));
        }

        let author = response.json::<Self>()?;
        Ok(author)
    }
}

impl TryFrom<reqwest::blocking::Response> for AuthorResponse {
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

impl Author {
    pub fn new(id: &str) -> Result<Self> {
        let url = format!("{}/A{}", API_URL, id);
        let response = reqwest::blocking::get(url)?;
        response.try_into()
    }

    pub fn get_samples(number_of_samples: u32, seed: impl Into<String>) -> Result<AuthorResponse> {
        let client = Client::new();
        let response = client
            .get(API_URL)
            .query(&[
                ("sample", number_of_samples.to_string()),
                ("seed", seed.into()),
            ])
            .send()?;
        response.try_into()
    }
}
