//! Determine which emails or Slack channels to forward to when receiving an Alert

use crate::resource::{Create, Delete, Resource};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct AccountAlert {
    /// Unique ID of the account_alert
    pub account_alert_id: u64,

    /// key representing the type of alert. For example bad_ad alert will send when Buzz detects a
    /// bad ad.
    pub system_alert_key: Option<String>,

    /// One or more emails to forward the alert to, separated by commas
    pub email: Option<String>,

    /// Slack API key
    pub slack_api: Option<String>,

    /// Slack channel such as #buzz-alerts
    pub slack_channel: Option<String>,

    /// Emoji to post next to alert in slack, such as :poop:
    pub slack_emoji: Option<String>,

    /// Is the Account Alert currently active
    pub active: Option<bool>,
}

impl Resource for AccountAlert {
    const NAME: &'static str = "account_alert";
}

// ToDo: Find out if there is a way to Read the account alert

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct CreateAccountAlert {
    /// key representing the type of alert. For example bad_ad alert will send when Buzz detects a
    /// bad ad.
    pub system_alert_key: String,

    /// One or more emails to forward the alert to, separated by commas
    pub email: Option<String>,

    /// Slack API key
    pub slack_api: Option<String>,

    /// Slack channel such as #buzz-alerts
    pub slack_channel: Option<String>,

    /// Emoji to post next to alert in slack, such as :poop:
    pub slack_emoji: Option<String>,

    /// Is the Account Alert currently active
    pub active: Option<bool>,
}

impl Create<AccountAlert> for CreateAccountAlert {
    fn into_resource(self, id: u64) -> AccountAlert {
        AccountAlert {
            account_alert_id: id,
            system_alert_key: Some(self.system_alert_key),
            email: self.email,
            slack_api: self.slack_api,
            slack_channel: self.slack_channel,
            slack_emoji: self.slack_emoji,
            active: self.active,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DeleteAccountAlert {
    /// Unique ID of the account_alert
    account_alert_id: u64,
}

impl Delete<AccountAlert> for DeleteAccountAlert {}
