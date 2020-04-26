use crate::resource::Resource;
use serde::{Deserialize, Serialize};

/// The authenticate API method allows you to login to Buzz, change your password and logout
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
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
}

impl Authenticate {
    /// Create a simple authentication object with a username and password
    /// See [Authentication](https://docs.beeswax.com/docs/authentication)
    /// ```rust
    /// use beeswax_rs::resource::Authenticate;
    ///
    /// let authentication = Authenticate::simple("example@example.com".to_string(), "password".to_string());
    /// assert_eq!(
    ///   authentication,
    ///   Authenticate {
    ///     email: "example@example.com".to_string(),
    ///     password: "password".to_string(),
    ///     account_id: None,
    ///     keep_logged_in: false,
    ///   }
    /// )
    /// ```
    pub fn simple(email: String, password: String) -> Authenticate {
        Authenticate {
            email,
            password,
            ..Default::default()
        }
    }
}
