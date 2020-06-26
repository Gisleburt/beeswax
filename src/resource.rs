//! A collection of resources to use with the api
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod account_alert;
pub mod advertiser;
mod any_resource;
pub mod authenticate;
pub mod campaign;
pub mod common;
pub mod creative;
pub mod creative_line_item;
pub mod line_item;
pub mod view;
pub mod view_list;

pub use account_alert::AccountAlert;
pub use advertiser::Advertiser;
pub use any_resource::{AnyResource, FromAnyResource};
pub use authenticate::Authenticate;
pub use campaign::Campaign;
pub use creative::Creative;
pub use creative_line_item::CreativeLineItem;
pub use line_item::LineItem;
pub use view::View;
pub use view_list::ViewList;

use crate::{AsyncBeeswaxClient, Result, SyncBeeswaxClient};
use std::fmt::Debug;
use crate::client::sync_client::SyncClient;

pub trait Resource:
    Clone + Debug + Serialize + DeserializeOwned + Sync + Into<AnyResource> + FromAnyResource
{
    const NAME: &'static str;

    fn update_sync<C: SyncClient>(&self, beeswax_client: &C) -> Result<&Self> {
        beeswax_client.update(self)
    }

    // async fn update_async(&self, beeswax_client: &AsyncBeeswaxClient) -> Result<Vec<R>> {
    //     beeswax_client.update(self)
    // }
}

pub trait Read<R: Resource>: Clone + Serialize + Sync + PartialEq<R> {
    fn read_sync<C: SyncClient>(&self, beeswax_client: &C) -> Result<Vec<R>> {
        beeswax_client.read(self)
    }

    // async fn read_async(&self, beeswax_client: &AsyncBeeswaxClient) -> Result<Vec<R>> {
    //     beeswax_client.read(self)
    // }
}

pub trait Create<R: Resource>: Clone + Serialize + Sync {
    fn into_resource(self, id: u64) -> R;

    fn create_sync<C: SyncClient>(&self, beeswax_client: &C) -> Result<R> {
        beeswax_client.create(self)
    }

    // async fn create_async(&self, beeswax_client: &AsyncBeeswaxClient) -> Result<R> {
    //     beeswax_client.create(self)
    // }
}

pub trait Delete<R: Resource>: Clone + Serialize + Sync {
    fn delete_sync<C: SyncClient>(&self, beeswax_client: &C) -> Result<()> {
        beeswax_client.delete(self)
    }

    // async fn delete_async(&self, beeswax_client: &AsyncBeeswaxClient) -> Result<()> {
    //     beeswax_client.delete(self).await
    // }
}

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
