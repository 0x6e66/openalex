use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

use crate::prelude::*;

use super::{
    common_types::{DehydratedInstitution, Field, Meta},
    filter::Filter,
    sort::Sort,
};

const API_URL: &str = "https://api.openalex.org/authors";

#[derive(Deserialize, Serialize, Debug)]
pub struct SummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    pub _2yr_mean_citedness: f32,
    pub h_index: u32,
    pub i10_index: u32,
}

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
pub struct CountByYear {
    pub year: u32,
    pub works_count: u32,
    pub cited_by_count: u32,
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
    pub affiliations: Vec<Affiliation>,
    pub cited_by_count: u32,
    pub counts_by_year: Vec<CountByYear>,
    // TODO: change for chrono::datatime
    pub created_date: String,
    pub display_name: String,
    pub display_name_alternatives: Vec<String>,
    pub id: String,
    pub ids: AuthorIds,
    pub last_known_institutions: Vec<DehydratedInstitution>,
    pub orcid: Option<String>,
    pub summary_stats: SummaryStats,
    // TODO: change for chrono::datatime
    pub topics: Vec<AuthorTopic>,
    pub updated_date: String,
    pub works_api_url: String,
    pub works_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorResponse {
    pub meta: Meta,
    pub results: Vec<Author>,
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

    pub fn filter(filter: Filter, page: u32, per_page: u32, sort: Sort) -> Result<AuthorResponse> {
        let client = Client::new();
        let response = client
            .get(API_URL)
            .query(&[
                ("filter", filter.to_string()),
                ("page", page.to_string()),
                ("per-page", per_page.to_string()),
                ("sort", sort.to_string()),
            ])
            .send()?;
        response.try_into()
    }
}
