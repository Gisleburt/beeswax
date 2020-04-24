use serde::{de::DeserializeOwned, Serialize};

mod authenticate;

pub use authenticate::Authenticate;

pub trait Resource: Serialize + DeserializeOwned {
    const NAME: &'static str;
    const ID_FIELD: &'static str;
}
