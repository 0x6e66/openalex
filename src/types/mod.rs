pub mod author;
pub mod common_types;
pub mod filter;
pub mod sort;
pub mod source;
pub mod work;

use crate::prelude::*;
use filter::Filter;
use reqwest::blocking::Response;
use sort::Sort;

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
                                "Error deserializing Work object. Original Message: {}",
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
    ResponseType: TryFrom<Response>,
    EntityType: TryFrom<Response>,
{
    const API_URL: &'static str;

    #[allow(clippy::new_ret_no_self)]
    fn new(id: &str) -> Result<EntityType>;
    fn get_samples(number_of_samples: u32, seed: impl Into<String>) -> Result<ResponseType>;
    fn filter(filter: Filter, page: u32, per_page: u32, sort: Sort) -> Result<ResponseType>;
    fn search(
        search: impl Into<String>,
        page: u32,
        per_page: u32,
        sort: Sort,
    ) -> Result<ResponseType>;
}
