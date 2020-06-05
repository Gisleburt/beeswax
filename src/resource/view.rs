//! The View API method allows you to flexibly GET data from the Buzz system. It is especially
//! useful for querying static tables to discover ad types. acceptable mime types, or other lookup
//! fields. Views must be created by the Buzz administrator. Only GET requests are supported.

use crate::resource::{common::ViewName, Read, Resource};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct View(JsonValue);

impl Resource for View {
    const NAME: &'static str = "view";
}

impl Deref for View {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Serialize)]
pub struct ReadView {
    pub view_name: ViewName,
}

impl Read<View> for ReadView {}

impl PartialEq<View> for ReadView {
    fn eq(&self, other: &View) -> bool {
        unreachable!("View is of unknown shape and should not get compared to ReadView")
    }
}
