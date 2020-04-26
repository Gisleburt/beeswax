use serde::{de::DeserializeOwned, Deserialize, Serialize};

//! A collection of resources to use with the api

mod advertiser;
mod authenticate;
mod creative;

pub use advertiser::*;
pub use authenticate::*;
pub use creative::*;

pub trait Resource: Serialize + DeserializeOwned {
    const NAME: &'static str;
    const ID_FIELD: &'static str;
}

pub trait SearchCriteria<T: Resource>: Serialize {}

#[derive(Deserialize)]
pub struct Response<R: Resource> {
    pub success: bool,
    #[serde(bound(deserialize = "Vec<R>: Deserialize<'de>"))]
    pub payload: Vec<R>,
}
