pub mod author;
pub mod funder;
pub mod common_types;
pub mod institution;
pub mod concept;
pub mod keyword;
pub mod publisher;
pub mod source;
pub mod topic;
pub mod work;

use reqwest::blocking::{Client, Response};

use crate::{
    prelude::*,
    utils::{filter::Filter, sort::Sort},
};

#[macro_export]
macro_rules! impl_try_from_for_single_entity {
    ($entity:ty) => {
        impl TryFrom<reqwest::blocking::Response> for $entity {
            type Error = $crate::error::Error;

            fn try_from(response: reqwest::blocking::Response) -> $crate::prelude::Result<Self> {
                match response.status() {
                    reqwest::StatusCode::OK => {
                        let res = response.json::<Self>();
                        match res {
                            Ok(entity) => Ok(entity),
                            Err(e) => Err($crate::error::Error::Generic(format!(
                                "Error deserializing object. Original Message: {}",
                                e
                            ))),
                        }
                    }
                    reqwest::StatusCode::NOT_FOUND => {
                        let oa_error = $crate::error::OpenAlexError {
                            error: "Document not found".to_string(),
                            message: "The document with the specified id was not found. HTTP 404"
                                .to_string(),
                        };
                        Err($crate::error::Error::OpenAlex(oa_error))
                    }
                    _ => Err($crate::error::Error::Generic(format!(
                        "Unknown Error. Response Code was {}",
                        response.status()
                    ))),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_for_entity_response {
    ($entity_response:ty) => {
        impl TryFrom<reqwest::blocking::Response> for $entity_response {
            type Error = $crate::error::Error;

            fn try_from(response: reqwest::blocking::Response) -> $crate::prelude::Result<Self> {
                match response.status() {
                    reqwest::StatusCode::OK => {
                        let res = response.json::<Self>();
                        match res {
                            Ok(author_response) => Ok(author_response),
                            Err(e) => Err($crate::error::Error::Generic(format!(
                                "Error deserializing Work object. Original Message: {}",
                                e
                            ))),
                        }
                    }
                    reqwest::StatusCode::FORBIDDEN => {
                        let res = response.json::<$crate::error::OpenAlexError>();
                        match res {
                            Ok(oa_error) => Err($crate::error::Error::OpenAlex(oa_error)),
                            Err(e) => Err($crate::error::Error::Generic(format!(
                                "Error deserializing OpenAlexError object. Original Message: {}",
                                e
                            ))),
                        }
                    }
                    _ => Err($crate::error::Error::Generic(format!(
                        "Unknown Error. Response Code was {}",
                        response.status()
                    ))),
                }
            }
        }
    };
}

pub trait APIEntity<EntityType, ResponseType>
where
    EntityType: TryFrom<Response, Error = Error>,
    ResponseType: TryFrom<Response, Error = Error>,
{
    const API_URL: &'static str;

    #[allow(clippy::new_ret_no_self)]
    fn new(id: &str) -> Result<EntityType> {
        let url = format!("{}/{}", Self::API_URL, id);
        let response = reqwest::blocking::get(url)?;
        response.try_into()
    }
    fn get_samples(number_of_samples: u32, seed: impl Into<String>) -> Result<ResponseType> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("sample", number_of_samples.to_string()),
                ("seed", seed.into()),
            ])
            .send()?;
        response.try_into()
    }
    fn filter(filter: Filter, page: u32, per_page: u32, sort: Sort) -> Result<ResponseType> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("filter", filter.to_string()),
                ("page", page.to_string()),
                ("per-page", per_page.to_string()),
                ("sort", sort.to_string()),
            ])
            .send()?;
        response.try_into()
    }

    fn search(
        search: impl Into<String>,
        page: u32,
        per_page: u32,
        sort: Sort,
    ) -> Result<ResponseType> {
        let client = Client::new();
        let response = client
            .get(Self::API_URL)
            .query(&[
                ("search", search.into()),
                ("page", page.to_string()),
                ("per-page", per_page.to_string()),
                ("sort", sort.to_string()),
            ])
            .send()?;

        response.try_into()
    }
}
