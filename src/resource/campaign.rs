use crate::resource::{Create, Delete, FrequencyCap, Read, Resource, RevenueType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Campaign {
    /// Unique ID of the campaign
    pub campaign_id: u64,

    /// Must belong to the same account as the Campaign and be active
    pub advertiser_id: u64,

    /// Name of the Campaign, e.g. "Winter lead generation"
    pub campaign_name: String,

    /// Maximum amount to spend on this Campaign
    pub campaign_bud: f64,

    /// Maximum amount to spend or deliver in a day, Cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign. Cannot be
    /// lower than the daily_budget for any Line Items associated with this campaign.
    pub daily_budget: Option<f64>,

    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    pub budget_type: Option<u64>,

    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    pub revenue_type: Option<RevenueType>,

    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    pub revenue_amount: Option<f64>,

    /// This field is deprecated. Pacing is available at the Line Item level.
    pub pacing: Option<u64>,

    /// ID of a Bid Modifier object to associate with the Campaign. If set, max_bid must also be
    /// set.
    pub bid_modifier_id: Option<u64>,

    /// ID of the Delivery Modifier to associate with Line Items under this Campaign
    pub delivery_modifier_id: Option<u64>,

    /// Maximum bid after taking into consideration any Bid Modifiers.
    pub max_bid: Option<f64>,

    /// Start date of the Campaign. No Line Items associated with the Campaign can have start dates
    /// prior to this date.
    pub start_date: String,

    /// End date of the Campaign. No Line Items associated with the Campaign can have end dates
    /// after this date. End date must be provided in order to pace.
    pub end_date: Option<String>,

    /// Frequency cap JSON.
    pub frequency_cap: Option<Vec<FrequencyCap>>,

    /// The method of frequency capping. All Line Items must match Campaign-level if set. For
    /// definitions, see the Frequency Cap guide.
    pub frequency_cap_type: Option<u64>,

    /// Continents in which the Campaign is eligible to serve. Inherited from the Advertiser object
    /// if left blank.
    pub continents: Option<Vec<String>>,

    /// Currency in which all Line Items under this Campaign will bid. Cannot be changed once set.
    /// If a default is set at the Advertiser level, it will be inherited here.
    pub currency: Option<String>,

    /// An alternative id to lookup the Campaign, if desired
    pub alternative_id: Option<String>,

    /// Notes, up to 255 chars
    pub notes: Option<String>,

    /// Is it active?
    pub active: bool,

    pub campaign_budget: u64,
    pub campaign_spend: u64,

    // ToDo: Unknown Type
    // pub test_plan_id: Unknown Type,
    // ToDo: Unknown Type
    // pub default_targeting: Unknown Type,
    pub push_status: u64,
    pub push_update: bool,
    pub account_id: u64,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub buzz_key: String,
}

impl Resource for Campaign {
    const NAME: &'static str = "resource";
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReadCampaign {
    /// Unique ID of the campaign
    campaign_id: Option<u64>,
    /// Id of the Advertiser the campaign belongs too
    advertiser_id: Option<u64>,
    /// Name of the campaign, e.g. "Winter lead generation"
    campaign_name: Option<String>,
    /// ID of the Bid Modifier associated with this Campaign
    bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier associated with this Campaign
    delivery_modifier_id: Option<u64>,
    /// An alternative id to lookup the campaign, if desired
    alternative_id: Option<String>,
    /// Is it active?
    active: Option<bool>,
    create_date: Option<String>,
    update_date: Option<String>,
}

impl Read<Campaign> for ReadCampaign {}

#[derive(Clone, Debug, Default, Serialize)]
struct CreateCampaign {
    /// Must belong to the same account as the Campaign and be active
    pub advertiser_id: u64,

    /// Name of the Campaign, e.g. "Winter lead generation"
    pub campaign_name: String,

    /// Maximum amount to spend on this Campaign
    pub campaign_bud: f64,

    /// Maximum amount to spend or deliver in a day, Cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign. Cannot be
    /// lower than the daily_budget for any Line Items associated with this campaign.
    pub daily_budget: Option<f64>,

    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    pub budget_type: Option<u64>,

    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    pub revenue_type: Option<RevenueType>,

    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    pub revenue_amount: Option<f64>,

    /// This field is deprecated. Pacing is available at the Line Item level.
    pub pacing: Option<u64>,

    /// ID of a Bid Modifier object to associate with the Campaign. If set, max_bid must also be
    /// set.
    pub bid_modifier_id: Option<u64>,

    /// ID of the Delivery Modifier to associate with Line Items under this Campaign
    pub delivery_modifier_id: Option<u64>,

    /// Maximum bid after taking into consideration any Bid Modifiers.
    pub max_bid: Option<f64>,

    /// Start date of the Campaign. No Line Items associated with the Campaign can have start dates
    /// prior to this date.
    pub start_date: String,

    /// End date of the Campaign. No Line Items associated with the Campaign can have end dates
    /// after this date. End date must be provided in order to pace.
    pub end_date: Option<String>,

    /// Frequency cap JSON.
    pub frequency_cap: Option<Vec<FrequencyCap>>,

    /// The method of frequency capping. All Line Items must match Campaign-level if set. For
    /// definitions, see the Frequency Cap guide.
    pub frequency_cap_type: Option<u64>,

    /// Continents in which the Campaign is eligible to serve. Inherited from the Advertiser object
    /// if left blank.
    pub continents: Option<Vec<String>>,

    /// Currency in which all Line Items under this Campaign will bid. Cannot be changed once set.
    /// If a default is set at the Advertiser level, it will be inherited here.
    pub currency: Option<String>,

    /// An alternative id to lookup the Campaign, if desired
    pub alternative_id: Option<String>,

    /// Notes, up to 255 chars
    pub notes: Option<String>,

    /// Is it active?
    pub active: bool,
}

impl Create<Campaign> for CreateCampaign {
    fn into_resource(self, campaign_id: u64) -> Campaign {
        Campaign {
            campaign_id,
            advertiser_id: self.advertiser_id,
            campaign_name: self.campaign_name,
            campaign_bud: self.campaign_bud,
            daily_budget: self.daily_budget,
            budget_type: self.budget_type,
            revenue_type: self.revenue_type,
            revenue_amount: self.revenue_amount,
            pacing: self.pacing,
            bid_modifier_id: self.bid_modifier_id,
            delivery_modifier_id: self.delivery_modifier_id,
            max_bid: self.max_bid,
            start_date: self.start_date,
            end_date: self.end_date,
            frequency_cap: self.frequency_cap,
            frequency_cap_type: self.frequency_cap_type,
            continents: self.continents,
            currency: self.currency,
            alternative_id: self.alternative_id,
            notes: self.notes,
            active: self.active,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct DeleteCampaign {
    campaign_id: u64,
}

impl Delete<Campaign> for DeleteCampaign {}

impl Delete<Campaign> for Campaign {}
