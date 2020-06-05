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

impl From<Authenticate> for AnyResource {
    fn from(r: Authenticate) -> Self {
        AnyResource::Authenticate(r)
    }
}
impl From<AccountAlert> for AnyResource {
    fn from(r: AccountAlert) -> Self {
        AnyResource::AccountAlert(r)
    }
}

impl From<Campaign> for AnyResource {
    fn from(r: Campaign) -> AnyResource {
        AnyResource::Campaign(r)
    }
}
impl From<Advertiser> for AnyResource {
    fn from(r: Advertiser) -> AnyResource {
        AnyResource::Advertiser(r)
    }
}
impl From<Creative> for AnyResource {
    fn from(r: Creative) -> AnyResource {
        AnyResource::Creative(r)
    }
}
impl From<CreativeLineItem> for AnyResource {
    fn from(r: CreativeLineItem) -> AnyResource {
        AnyResource::CreativeLineItem(r)
    }
}
impl From<LineItem> for AnyResource {
    fn from(r: LineItem) -> AnyResource {
        AnyResource::LineItem(r)
    }
}
impl From<View> for AnyResource {
    fn from(r: View) -> AnyResource {
        AnyResource::View(r)
    }
}
impl From<ViewList> for AnyResource {
    fn from(r: ViewList) -> AnyResource {
        AnyResource::ViewList(r)
    }
}

impl FromAnyResource for AccountAlert {
    fn from_any_resource(ar: &AnyResource) -> Option<&AccountAlert> {
        match ar {
            AnyResource::AccountAlert(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for Advertiser {
    fn from_any_resource(ar: &AnyResource) -> Option<&Advertiser> {
        match ar {
            AnyResource::Advertiser(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for Authenticate {
    fn from_any_resource(ar: &AnyResource) -> Option<&Authenticate> {
        match ar {
            AnyResource::Authenticate(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for Campaign {
    fn from_any_resource(ar: &AnyResource) -> Option<&Campaign> {
        match ar {
            AnyResource::Campaign(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for Creative {
    fn from_any_resource(ar: &AnyResource) -> Option<&Creative> {
        match ar {
            AnyResource::Creative(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for CreativeLineItem {
    fn from_any_resource(ar: &AnyResource) -> Option<&CreativeLineItem> {
        match ar {
            AnyResource::CreativeLineItem(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for LineItem {
    fn from_any_resource(ar: &AnyResource) -> Option<&LineItem> {
        match ar {
            AnyResource::LineItem(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for View {
    fn from_any_resource(ar: &AnyResource) -> Option<&View> {
        match ar {
            AnyResource::View(r) => Some(r),
            _ => None,
        }
    }
}
impl FromAnyResource for ViewList {
    fn from_any_resource(ar: &AnyResource) -> Option<&ViewList> {
        match ar {
            AnyResource::ViewList(r) => Some(r),
            _ => None,
        }
    }
}
