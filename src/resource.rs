//! A collection of resources to use with the api
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod account_alerts;
pub mod advertiser;
pub mod authenticate;
pub mod campaign;
pub mod common;
pub mod creative;
pub mod creative_line_item;
pub mod line_item;
pub mod view;
pub mod view_list;

use std::fmt::Debug;

pub trait Resource: Clone + Debug + Serialize + DeserializeOwned {
    const NAME: &'static str;
}

pub trait Read<R: Resource>: Clone + Serialize {}

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
