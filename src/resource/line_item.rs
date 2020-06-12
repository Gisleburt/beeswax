//! A Line Item object defines the objectives, budget and timing for a portion of a Campaign. Line
//! Items belong to a single Campaign, may be associated with one or more Creatives using the
//! Creative Line Item Association, and are associated with a single Targeting Template.

use crate::resource::{
    common::{BiddingStratergy, FrequencyCap, RevenueType, WeightingMethod},
    Create, Delete, Read, Resource,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LineItem {
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
    create_date: Option<String>,
    update_date: Option<String>,
    buzz_key: String,
}

impl LineItem {
    /// Create a builder for CreateLineItem
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::LineItem;
    ///
    /// let create_advertiser = LineItem::create_builder()
    ///     .advertiser_id(1)
    ///     .campaign_id(2)
    ///     .line_item_type_id(3)
    ///     .line_item_budget(1000.0)
    ///     .build();
    ///
    /// let advertiser = beeswax_client.create(&create_advertiser).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_builder() -> CreateLineItemBuilder<(
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
        (),
    )> {
        CreateLineItem::builder()
    }

    /// Create a builder for ReadLineItem
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::LineItem;
    ///
    /// let read_line_item = LineItem::read_builder()
    ///     .advertiser_id(1)
    ///     .build();
    ///
    /// let line_items = beeswax_client.read(&read_line_item).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_builder(
    ) -> ReadLineItemBuilder<((), (), (), (), (), (), (), (), (), (), (), (), ())> {
        ReadLineItem::builder()
    }

    /// Create a builder for DeleteLineItem
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::LineItem;
    ///
    /// let delete_line_item = LineItem::delete_builder()
    ///     .line_item_id(10)
    ///     .build();
    ///
    /// beeswax_client.delete(&delete_line_item).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_builder() -> DeleteLineItemBuilder<((),)> {
        DeleteLineItem::builder()
    }
}

impl Resource for LineItem {
    const NAME: &'static str = "line_item";
}

#[derive(Clone, Default, Debug, Serialize, TypedBuilder)]
pub struct ReadLineItem {
    /// Unique ID of the Line Item
    #[builder(default, setter(into))]
    line_item_id: Option<u64>,
    /// Must be a valid and active Campaign
    #[builder(default, setter(into))]
    campaign_id: Option<u64>,
    /// Must belong to the same account as the Line Item and be active
    #[builder(default, setter(into))]
    advertiser_id: Option<u64>,
    /// The type of the Line Item. 0=banner, 1=video.
    #[builder(default, setter(into))]
    line_item_type_id: Option<u64>,
    /// Name of the Line Item, e.g. "Winter lead generation"
    #[builder(default, setter(into))]
    line_item_name: Option<String>,
    /// ID of the Bid Modifier associated with this Line Item
    #[builder(default, setter(into))]
    bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier associated with this Line Item
    #[builder(default, setter(into))]
    delivery_modifier_id: Option<u64>,
    /// Start date of the Line Item. No Line Items associated with the Campaign can have start dates prior to this date.
    #[builder(default, setter(into))]
    start_date: Option<String>,
    /// End date of the Line Item. No Line Items associated with the Campaign can have end dates after this date.
    #[builder(default, setter(into))]
    end_date: Option<String>,
    /// An alternative id to lookup the Line Item, if desired
    #[builder(default, setter(into))]
    alternative_id: Option<String>,
    #[builder(default, setter(into))]
    active: Option<bool>,
    #[builder(default, setter(into))]
    create_date: Option<String>,
    #[builder(default, setter(into))]
    update_date: Option<String>,
}

impl Read<LineItem> for ReadLineItem {}

impl PartialEq<LineItem> for ReadLineItem {
    fn eq(&self, other: &LineItem) -> bool {
        (self.line_item_id.is_none() || self.line_item_id == Some(other.line_item_id))
            && (self.campaign_id.is_none() || self.campaign_id == Some(other.campaign_id))
            && (self.advertiser_id.is_none() || self.advertiser_id == Some(other.advertiser_id))
            && (self.line_item_type_id.is_none()
                || self.line_item_type_id == Some(other.line_item_type_id))
            && (self.line_item_name.is_none()
                || self.line_item_name.as_ref() == Some(&other.line_item_name))
            && (self.bid_modifier_id.is_none() || self.bid_modifier_id == other.bid_modifier_id)
            && (self.delivery_modifier_id.is_none()
                || self.delivery_modifier_id == other.delivery_modifier_id)
            && (self.start_date.is_none() || self.start_date.as_ref() == Some(&other.start_date))
            && (self.end_date.is_none() || self.end_date == other.end_date)
            && (self.alternative_id.is_none() || self.alternative_id == other.alternative_id)
            && (self.active.is_none() || self.active == Some(other.active))
            && (self.create_date.is_none() || self.create_date == other.create_date)
            && (self.update_date.is_none() || self.update_date == other.update_date)
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, TypedBuilder)]
pub struct CreateLineItem {
    /// Must be a valid and active Campaign
    pub campaign_id: u64,
    /// Must be active
    pub advertiser_id: u64,
    /// The type of the Line Item. 0=banner, 1=video, 2=native
    pub line_item_type_id: u64,
    /// The ID of the associated Targeting Template, must be a valid and active Targeting Template.
    #[builder(default, setter(into))]
    pub targeting_template_id: Option<u64>,
    /// Name of the Line Item, e.g. "Winter lead generation"
    #[builder(default, setter(into))]
    pub line_item_name: String,
    /// Maximum amount to spend on this Line Item
    pub line_item_budget: f64,
    /// Maximum amount to spend or deliver in a day, cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign.
    #[builder(default, setter(into))]
    pub daily_budget: Option<f64>,
    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    #[builder(default, setter(into))]
    pub budget_type: Option<u64>,
    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    #[builder(default, setter(into))]
    pub revenue_type: Option<RevenueType>,
    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    #[builder(default, setter(into))]
    pub revenue_amount: Option<f64>,
    /// ID of a Bid Modifier object to associate with the Line Item. If set, max_bid must also be
    /// set.
    #[builder(default, setter(into))]
    pub bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier to associate with this Line Item
    #[builder(default, setter(into))]
    pub delivery_modifier_id: Option<u64>,
    /// Maximum bid after taking into consideration any Bid Modifiers.
    #[builder(default, setter(into))]
    pub max_bid: Option<f64>,
    /// Bidding Strategy JSON.
    #[builder(default, setter(into))]
    pub bidding: BiddingStratergy,
    /// Either RANDOM or WEIGHTED
    #[builder(default, setter(into))]
    pub creative_weighting_method: Option<WeightingMethod>,
    /// When using Experiments the group to use for segregating users. The test_group_id must belong
    /// to the test_plan_id assigned to the Campaign. If a test_plan_id is set at the Campaign
    /// level, the test_group_id may not be null.
    #[builder(default, setter(into))]
    pub test_group_id: Option<u64>,
    /// Start date of the Line Item.
    #[builder(default, setter(into))]
    pub start_date: String,
    /// End date of the Line Item.
    #[builder(default, setter(into))]
    pub end_date: Option<String>,
    /// Frequency cap JSON.
    #[builder(default, setter(into))]
    pub frequency_cap: Option<Vec<FrequencyCap>>,
    /// The method of frequency capping. Must match Campaign-level if set. For definitions, see the
    /// Frequency Cap guide.
    #[builder(default, setter(into))]
    pub frequency_cap_type: Option<u64>,
    /// When targeting by user_time_of_week, this field should include a list of timezones you
    /// expect the ad to serve within. If not set properly, pacing will be uneven.
    #[builder(default, setter(into))]
    pub user_timezones: Option<Vec<String>>,
    /// An alternative id to lookup the Line Item, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,
    /// Notes about the Line Item, up to 255 chars
    #[builder(default, setter(into))]
    pub notes: Option<String>,
    /// Is the Line Item active? Must be set to 0 on POST since no Creatives are yet assigned.
    #[builder(default)]
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

#[derive(Clone, Debug, Serialize, TypedBuilder)]
pub struct DeleteLineItem {
    line_item_id: u64,
}

impl Delete<LineItem> for DeleteLineItem {}

impl Delete<LineItem> for LineItem {}
