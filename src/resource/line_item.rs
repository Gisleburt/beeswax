use crate::resource::{
    BiddingStratergy, Create, Delete, Read, FrequencyCap, Resource, RevenueType, WeightingMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct LineItem {
    line_item_id: u64,
    /// Must be a valid and active Campaign
    campaign_id: u64,
    /// Must be active
    advertiser_id: u64,
    /// The type of the Line Item. 0=banner, 1=video, 2=native
    line_item_type_id: u64,
    /// The ID of the associated Targeting Template, must be a valid and active Targeting Template.
    targeting_template_id: Option<u64>,
    /// Name of the Line Item, e.g. "Winter lead generation"
    line_item_name: String,
    /// Maximum amount to spend on this Line Item
    line_item_budget: f64,
    /// Maximum amount to spend or deliver in a day, cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign.
    daily_budget: Option<f64>,
    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    budget_type: Option<u64>,
    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    revenue_type: Option<RevenueType>,
    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    revenue_amount: Option<f64>,
    /// ID of a Bid Modifier object to associate with the Line Item. If set, max_bid must also be
    /// set.
    bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier to associate with this Line Item
    delivery_modifier_id: Option<u64>,
    /// Maximum bid after taking into consideration any Bid Modifiers.
    max_bid: Option<f64>,
    /// Bidding Strategy JSON.
    bidding: BiddingStratergy,
    /// Either RANDOM or WEIGHTED
    creative_weighting_method: Option<WeightingMethod>,
    /// When using Experiments the group to use for segregating users. The test_group_id must belong
    /// to the test_plan_id assigned to the Campaign. If a test_plan_id is set at the Campaign
    /// level, the test_group_id may not be null.
    test_group_id: Option<u64>,
    /// Start date of the Line Item.
    start_date: String,
    /// End date of the Line Item.
    end_date: Option<String>,
    /// Frequency cap JSON.
    frequency_cap: Option<Vec<FrequencyCap>>,
    /// The method of frequency capping. Must match Campaign-level if set. For definitions, see the
    /// Frequency Cap guide.
    frequency_cap_type: Option<u64>,
    /// When targeting by user_time_of_week, this field should include a list of timezones you
    /// expect the ad to serve within. If not set properly, pacing will be uneven.
    user_timezones: Option<Vec<String>>,
    /// An alternative id to lookup the Line Item, if desired
    alternative_id: Option<String>,
    /// Notes about the Line Item, up to 255 chars
    notes: Option<String>,
    /// Is the Line Item active? Must be set to 0 on POST since no Creatives are yet assigned.
    active: bool,

    line_item_version: u64,
    line_item_spend: u64,
    currency: String,
    // ToDo: Unknown Type
    // pacing: null,
    // ToDo: Unknown Type
    // test_plan_id: null,
    push_status: u64,
    push_update: bool,
    account_id: u64,
    create_date: String,
    update_date: String,
    buzz_key: String,
}

impl Resource for LineItem {
    const NAME: &'static str = "line_item";
}

#[derive(Clone, Default, Debug, Serialize)]
struct ReadLineItem {
    /// Unique ID of the Line Item
    line_item_id: Option<u64>,
    /// Must be a valid and active Campaign
    campaign_id: Option<u64>,
    /// Must belong to the same account as the Line Item and be active
    advertiser_id: Option<u64>,
    /// The type of the Line Item. 0=banner, 1=video.
    line_item_type_id: Option<u64>,
    /// Name of the Line Item, e.g. "Winter lead generation"
    line_item_name: Option<String>,
    /// ID of the Bid Modifier associated with this Line Item
    bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier associated with this Line Item
    delivery_modifier_id: Option<u64>,
    /// Start date of the Line Item. No Line Items associated with the Campaign can have start dates prior to this date.
    start_date: Option<String>,
    /// End date of the Line Item. No Line Items associated with the Campaign can have end dates after this date.
    end_date: Option<String>,
    /// An alternative id to lookup the Line Item, if desired
    alternative_id: Option<String>,
    active: Option<bool>,
    create_date: Option<String>,
    update_date: Option<String>,
}

impl Read<LineItem> for ReadLineItem {}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
struct CreateLineItem {
    /// Must be a valid and active Campaign
    pub campaign_id: u64,
    /// Must be active
    pub advertiser_id: u64,
    /// The type of the Line Item. 0=banner, 1=video, 2=native
    pub line_item_type_id: u64,
    /// The ID of the associated Targeting Template, must be a valid and active Targeting Template.
    pub targeting_template_id: Option<u64>,
    /// Name of the Line Item, e.g. "Winter lead generation"
    pub line_item_name: String,
    /// Maximum amount to spend on this Line Item
    pub line_item_budget: f64,
    /// Maximum amount to spend or deliver in a day, cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign.
    pub daily_budget: Option<f64>,
    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    pub budget_type: Option<u64>,
    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    pub revenue_type: Option<RevenueType>,
    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    pub revenue_amount: Option<f64>,
    /// ID of a Bid Modifier object to associate with the Line Item. If set, max_bid must also be
    /// set.
    pub bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier to associate with this Line Item
    pub delivery_modifier_id: Option<u64>,
    /// Maximum bid after taking into consideration any Bid Modifiers.
    pub max_bid: Option<f64>,
    /// Bidding Strategy JSON.
    pub bidding: BiddingStratergy,
    /// Either RANDOM or WEIGHTED
    pub creative_weighting_method: Option<WeightingMethod>,
    /// When using Experiments the group to use for segregating users. The test_group_id must belong
    /// to the test_plan_id assigned to the Campaign. If a test_plan_id is set at the Campaign
    /// level, the test_group_id may not be null.
    pub test_group_id: Option<u64>,
    /// Start date of the Line Item.
    pub start_date: String,
    /// End date of the Line Item.
    pub end_date: Option<String>,
    /// Frequency cap JSON.
    pub frequency_cap: Option<Vec<FrequencyCap>>,
    /// The method of frequency capping. Must match Campaign-level if set. For definitions, see the
    /// Frequency Cap guide.
    pub frequency_cap_type: Option<u64>,
    /// When targeting by user_time_of_week, this field should include a list of timezones you
    /// expect the ad to serve within. If not set properly, pacing will be uneven.
    pub user_timezones: Option<Vec<String>>,
    /// An alternative id to lookup the Line Item, if desired
    pub alternative_id: Option<String>,
    /// Notes about the Line Item, up to 255 chars
    pub notes: Option<String>,
    /// Is the Line Item active? Must be set to 0 on POST since no Creatives are yet assigned.
    pub active: bool,
}

impl Create<LineItem> for CreateLineItem {
    fn into_resource(self, line_item_id: u64) -> LineItem {
        LineItem {
            line_item_id,
            campaign_id: self.campaign_id,
            advertiser_id: self.advertiser_id,
            line_item_type_id: self.line_item_type_id,
            targeting_template_id: self.targeting_template_id,
            line_item_name: self.line_item_name,
            line_item_budget: self.line_item_budget,
            daily_budget: self.daily_budget,
            budget_type: self.budget_type,
            revenue_type: self.revenue_type,
            revenue_amount: self.revenue_amount,
            bid_modifier_id: self.bid_modifier_id,
            delivery_modifier_id: self.delivery_modifier_id,
            max_bid: self.max_bid,
            bidding: self.bidding,
            creative_weighting_method: self.creative_weighting_method,
            test_group_id: self.test_group_id,
            start_date: self.start_date,
            end_date: self.end_date,
            frequency_cap: self.frequency_cap,
            frequency_cap_type: self.frequency_cap_type,
            user_timezones: self.user_timezones,
            alternative_id: self.alternative_id,
            notes: self.notes,
            active: self.active,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct DeleteLineItem {
    creative_id: u64,
}

impl Delete<LineItem> for DeleteLineItem {}

impl Delete<LineItem> for LineItem {}
