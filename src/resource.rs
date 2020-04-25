use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod advertiser;
mod authenticate;

pub use advertiser::*;
pub use authenticate::*;

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
