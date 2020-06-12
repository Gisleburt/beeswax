//! The View List API method allows you to GET metadata about how a View and the fields within that
//! View should be displayed in a UI. There is no write API for this data, it must be manually
//! populated in SQL by an administrator. The primary use case for this API call is to display the
//! data from a view in a user interface. Only GET requests are supported.

use crate::resource::common::ViewName;
use crate::resource::{Read, Resource};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ViewList(JsonValue);

impl Resource for ViewList {
    const NAME: &'static str = "view_list";
}

impl Deref for ViewList {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Serialize)]
pub struct ReadViewList {
    pub view_name: ViewName,
}

impl Read<ViewList> for ReadViewList {}

impl PartialEq<ViewList> for ReadViewList {
    fn eq(&self, _other: &ViewList) -> bool {
        unreachable!("ViewList is of unknown shape and should not get compared to ReadViewList")
    }
}
