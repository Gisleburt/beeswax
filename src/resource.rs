//! A collection of resources to use with the api
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod advertiser;
mod authenticate;
mod creative;

pub use advertiser::*;
pub use authenticate::*;
pub use creative::*;
use std::fmt::Debug;

pub trait Resource: Debug + DeserializeOwned {
    const NAME: &'static str;
    const ID_FIELD: &'static str;
}

pub trait SearchCriteria<T: Resource>: Serialize {}

pub trait Create<T: Resource>: Serialize {}

#[derive(Debug, Deserialize)]
pub struct Response<R: Resource> {
    pub success: bool,
    #[serde(bound(deserialize = "Vec<R>: Deserialize<'de>"))]
    pub payload: Vec<R>,
}
