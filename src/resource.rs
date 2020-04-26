//! A collection of resources to use with the api
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod advertiser;
mod authenticate;
mod creative;

pub use advertiser::*;
pub use authenticate::*;
pub use creative::*;
use std::fmt::Debug;

pub trait Resource: Clone + Debug + Serialize + DeserializeOwned {
    const NAME: &'static str;
    const ID_FIELD: &'static str;
}

pub trait Search<R: Resource>: Clone + Serialize {}

pub trait Create<R: Resource>: Clone + Serialize {
    fn into_resource(self, id: u64) -> R;
}

pub trait Delete<R: Resource>: Clone + Serialize {}

#[derive(Debug, Deserialize)]
pub struct ResponseResource<R: Resource> {
    pub success: bool,
    #[serde(bound(deserialize = "Vec<R>: Deserialize<'de>"))]
    pub payload: Vec<R>,
}

#[derive(Debug, Deserialize)]
pub struct Id {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct ResponseId {
    pub success: bool,
    pub payload: Id,
}
