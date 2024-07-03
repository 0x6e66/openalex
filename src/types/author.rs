use reqwest::{blocking::Client, StatusCode};
use serde_derive::{Deserialize, Serialize};

use crate::{error::OpenAlexError, prelude::*};

use super::{
    common_types::{CountByYear, DehydratedInstitution, Field, Meta, SummaryStats},
    filter::Filter,
    sort::Sort,
};

const API_URL: &str = "https://api.openalex.org/authors";

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
        match response.status() {
            StatusCode::OK => {
                let res = response.json::<Self>();
                match res {
                    Ok(author) => Ok(author),
                    Err(e) => Err(Error::Generic(format!(
                        "Error deserializing Author object. Original Message: {}",
                        e
                    ))),
                }
            }
            StatusCode::NOT_FOUND => {
                let oa_error = OpenAlexError {
                    error: "Document not found".to_string(),
                    message: "The document with the specified id was not found. HTTP 404"
                        .to_string(),
                };
                Err(Error::OpenAlex(oa_error))
            }
            _ => Err(Error::Generic(format!(
                "Unknown Error. Response Code was {}",
                response.status()
            ))),
        }
    }
}

impl TryFrom<reqwest::blocking::Response> for AuthorResponse {
    type Error = Error;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self> {
        match response.status() {
            StatusCode::OK => {
                let res = response.json::<Self>();
                match res {
                    Ok(author_response) => Ok(author_response),
                    Err(e) => Err(Error::Generic(format!(
                        "Error deserializing Work object. Original Message: {}",
                        e
                    ))),
                }
            }
            StatusCode::FORBIDDEN => {
                let res = response.json::<OpenAlexError>();
                match res {
                    Ok(oa_error) => Err(Error::OpenAlex(oa_error)),
                    Err(e) => Err(Error::Generic(format!(
                        "Error deserializing OpenAlexError object. Original Message: {}",
                        e
                    ))),
                }
            }
            _ => Err(Error::Generic(format!(
                "Unknown Error. Response Code was {}",
                response.status()
            ))),
        }
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
