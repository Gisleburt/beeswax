use crate::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Authenticate {
    /// User Email
    pub email: String,
    /// User Password
    pub password: String,
    /// For Multi-Account Users only, allows the user to authenticate into a different account on
    /// the same Buzz instance
    pub account_id: Option<i32>,
    /// When set to true, the user will be kept logged in for up to 30 days
    pub keep_logged_in: bool,
}

impl Resource for Authenticate {
    const NAME: &'static str = "authenticate";
    const ID_FIELD: &'static str = "email";
}

impl Authenticate {
    pub fn simple(email: String, password: String) -> Authenticate {
        Authenticate {
            email,
            password,
            ..Default::default()
        }
    }
}
