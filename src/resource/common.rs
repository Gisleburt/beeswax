use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ConversionMethod {
    LastClick = 1,
}

impl Default for ConversionMethod {
    fn default() -> Self {
        ConversionMethod::LastClick
    }
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CreativeType {
    Banner = 0,
    Video = 1,
    Native = 2,
}

impl Default for CreativeType {
    fn default() -> Self {
        CreativeType::Banner
    }
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BudgetType {
    Spend = 0,
    Impressions = 1,
    SpendWithVendorFees = 2,
}

impl Default for BudgetType {
    fn default() -> Self {
        BudgetType::Spend
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FrequencyCap {
    /// Duration of time in which to cap impressions, in seconds. 30-day (2592000) max.
    pub duration: Option<u64>,

    /// Number of impressions to allow within the duration set
    pub impressions: Option<u64>,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
/// Beeswax supports flexible ways to use inbound auction data to frequency cap. Which method is
/// used depends on the frequency_cap_type field defined at the Campaign and Line Item level.
/// Important note: Not all types are supported by default and your account may need to be enabled
/// for some of the more advanced types. Please reach out to your account manager or support for
/// more details.
pub enum FrequencyCapType {
    /// Use the browser cookie OR the device ID to cap.
    Standard = 0,
    /// Use the IP address of the end-user as included in the bid request. Ignore values where the
    /// final octet is truncated or where due to GDPR or other privacy regulation we do not have
    /// consent to use the full IP address.
    IpAddress = 1,
    /// Use the browser cookie or device ID when present, then fall back to IP address when not
    /// present.
    StandardWithFallback = 2,
    /// Use an auction-provided customer ID when present, then fall back to the Standard option when
    /// customer ID is not present.
    CustomerIdWithFallback = 3,
}

impl Default for FrequencyCapType {
    fn default() -> Self {
        FrequencyCapType::Standard
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum RevenueType {
    CPM,
    CPC,
    CPCV,
    CPI,
    CPA,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WeightingMethod {
    RANDOM,
    WEIGHTED,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BiddingStratergy {
    /// The strategy to use, for example CPM.
    bidding_strategy: Option<String>,
    /// Set of keys and values specific to the bidding_strategy
    /// Each bidding_strategy supports a set of up to five keys, each of which should be passed
    /// with a value in the correct format. Values may be ints or strings. Example "cpm_bid":1.21
    values: Option<HashMap<String, f64>>,
    /// Whether the bidding_strategy is a custom strategy to your account. These are set up by the
    /// administrator.
    custom: Option<bool>,
    /// If a bidding strategy can be paced, the pacing value can be set to daily, flight, or
    /// lifetime. If it cannot be paced, then it should be set to none. The flight type may only be
    /// used if the Line Item has Line Item Flights enabled and every flight includes a budget.
    pacing: Option<String>,
}
