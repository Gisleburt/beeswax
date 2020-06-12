//! A collection of resources to use with the api
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod account_alert;
pub mod advertiser;
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
pub use authenticate::Authenticate;
pub use campaign::Campaign;
pub use creative::Creative;
pub use creative_line_item::CreativeLineItem;
pub use line_item::LineItem;
pub use view::View;
pub use view_list::ViewList;

use std::fmt::Debug;

pub trait Resource:
    Clone + Debug + Serialize + DeserializeOwned + Sync + Into<AnyResource> + FromAnyResource
{
    const NAME: &'static str;
}

pub trait Read<R: Resource>: Clone + Serialize + Sync + PartialEq<R> {}

pub trait Create<R: Resource>: Clone + Serialize + Sync {
    fn into_resource(self, id: u64) -> R;
}

pub trait Delete<R: Resource>: Clone + Serialize + Sync {}

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

#[derive(Clone, Debug)]
pub enum AnyResource {
    AccountAlert(AccountAlert),
    Advertiser(Advertiser),
    Authenticate(Authenticate),
    Campaign(Campaign),
    Creative(Creative),
    CreativeLineItem(CreativeLineItem),
    LineItem(LineItem),
    View(View),
    ViewList(ViewList),
}

pub trait FromAnyResource {
    fn from_any_resource(r: &AnyResource) -> Option<&Self>;
}

macro_rules! any_resource {
    ($i:ident) => {
        impl FromAnyResource for $i {
            fn from_any_resource(ar: &AnyResource) -> Option<&$i> {
                match ar {
                    AnyResource::$i(r) => Some(r),
                    _ => None,
                }
            }
        }

        impl From<$i> for AnyResource {
            fn from(r: $i) -> Self {
                AnyResource::$i(r)
            }
        }
    };
}

any_resource!(AccountAlert);
any_resource!(Advertiser);
any_resource!(Authenticate);
any_resource!(Campaign);
any_resource!(Creative);
any_resource!(CreativeLineItem);
any_resource!(LineItem);
any_resource!(View);
any_resource!(ViewList);
