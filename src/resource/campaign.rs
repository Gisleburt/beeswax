//! A Campaign object defines the overall budget and timing for one or more Line Items. Campaigns
//! belong to a single Advertisers.

use crate::resource::{
    common::{BudgetType, Continent, Currency, FrequencyCap, FrequencyCapType, RevenueType},
    Create, Delete, Read, Resource,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Campaign {
    /// Unique ID of the campaign
    pub campaign_id: u64,

    /// Must belong to the same account as the Campaign and be active
    pub advertiser_id: u64,

    /// Name of the Campaign, e.g. "Winter lead generation"
    pub campaign_name: String,

    /// Maximum amount to spend on this Campaign
    pub campaign_budget: f64,

    /// Maximum amount to spend or deliver in a day, Cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign. Cannot be
    /// lower than the daily_budget for any Line Items associated with this campaign.
    pub daily_budget: Option<f64>,

    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    pub budget_type: Option<BudgetType>,

    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    pub revenue_type: Option<RevenueType>,

    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    pub revenue_amount: Option<f64>,

    // This field is deprecated. Pacing is available at the Line Item level.
    // pub pacing: Option<u64>,
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
    pub frequency_cap_type: Option<FrequencyCapType>,

    /// Continents in which the Campaign is eligible to serve. Inherited from the Advertiser object
    /// if left blank.
    pub continents: Option<Vec<Continent>>,

    /// Currency in which all Line Items under this Campaign will bid. Cannot be changed once set.
    /// If a default is set at the Advertiser level, it will be inherited here.
    pub currency: Option<Currency>,

    /// An alternative id to lookup the Campaign, if desired
    pub alternative_id: Option<String>,

    /// Notes, up to 255 chars
    pub notes: Option<String>,

    /// Is it active?
    pub active: bool,

    pub campaign_spend: f64,

    // ToDo: Unknown Type
    // pub test_plan_id: Unknown Type,
    // ToDo: Unknown Type
    // pub default_targeting: Unknown Type,
    pub push_status: u64,
    // ToDo: The docs are wrong about this type
    pub push_update: u64,
    pub account_id: u64,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub buzz_key: String,
}

impl Campaign {
    /// Create a builder for CreateCampaign
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Campaign;
    ///
    /// let create_campaign = Campaign::create_builder()
    ///     .campaign_name("Some name")
    ///     .advertiser_id(1)
    ///     .build();
    ///
    /// let campaign = beeswax_client.create(&create_campaign).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_builder() -> CreateCampaignBuilder<(
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
        CreateCampaign::builder()
    }

    /// Create a builder for ReadCampaign
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Campaign;
    ///
    /// let read_campaign = Campaign::read_builder()
    ///     .campaign_name("Some name".to_string())
    ///     .build();
    ///
    /// let campaigns = beeswax_client.read(&read_campaign).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_builder() -> ReadCampaignBuilder<((), (), (), (), (), (), (), (), ())> {
        ReadCampaign::builder()
    }

    /// Create a builder for DeleteCampaign
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Campaign;
    ///
    /// let delete_campaign = Campaign::delete_builder()
    ///     .campaign_id(10)
    ///     .build();
    ///
    /// beeswax_client.delete(&delete_campaign).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_builder() -> DeleteCampaignBuilder<((),)> {
        DeleteCampaign::builder()
    }
}

impl Resource for Campaign {
    const NAME: &'static str = "campaign";
}

#[derive(Clone, Debug, Default, Serialize, TypedBuilder)]
pub struct ReadCampaign {
    /// Unique ID of the campaign
    #[builder(default)]
    pub campaign_id: Option<u64>,
    /// Id of the Advertiser the campaign belongs too
    #[builder(default, setter(into))]
    pub advertiser_id: Option<u64>,
    /// Name of the campaign, e.g. "Winter lead generation"
    #[builder(default, setter(into))]
    pub campaign_name: Option<String>,
    /// ID of the Bid Modifier associated with this Campaign
    #[builder(default, setter(into))]
    pub bid_modifier_id: Option<u64>,
    /// ID of the Delivery Modifier associated with this Campaign
    #[builder(default, setter(into))]
    pub delivery_modifier_id: Option<u64>,
    /// An alternative id to lookup the campaign, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,
    /// Is it active?
    #[builder(default, setter(into))]
    pub active: Option<bool>,
    #[builder(default, setter(into))]
    pub create_date: Option<String>,
    #[builder(default, setter(into))]
    pub update_date: Option<String>,
}

impl PartialEq<Campaign> for ReadCampaign {
    fn eq(&self, other: &Campaign) -> bool {
        (self.campaign_id.is_none() || self.campaign_id == Some(other.campaign_id))
            && (self.advertiser_id.is_none() || self.advertiser_id == Some(other.advertiser_id))
            && (self.campaign_name.is_none()
                || self.campaign_name.as_ref() == Some(&other.campaign_name))
            && (self.bid_modifier_id.is_none() || self.bid_modifier_id == other.bid_modifier_id)
            && (self.delivery_modifier_id.is_none()
                || self.delivery_modifier_id == other.delivery_modifier_id)
            && (self.alternative_id.is_none() || self.alternative_id == other.alternative_id)
            && (self.active.is_none() || self.active == Some(other.active))
            && (self.create_date.is_none() || self.create_date == other.create_date)
            && (self.update_date.is_none() || self.update_date == other.update_date)
    }
}

impl Read<Campaign> for ReadCampaign {}

#[derive(Clone, Debug, Default, Serialize, TypedBuilder)]
pub struct CreateCampaign {
    /// Must belong to the same account as the Campaign and be active
    pub advertiser_id: u64,

    /// Name of the Campaign, e.g. "Winter lead generation"
    #[builder(setter(into))]
    pub campaign_name: String,

    /// Maximum amount to spend on this Campaign
    #[builder(default)]
    pub campaign_budget: f64,

    /// Maximum amount to spend or deliver in a day, Cannot exceed campaign_budget or be so low as
    /// to prevent campaign_budget from being reached over the length of the campaign. Cannot be
    /// lower than the daily_budget for any Line Items associated with this campaign.
    #[builder(default, setter(into))]
    pub daily_budget: Option<f64>,

    /// Type of budget, 0=spend, 1=impressions, 2=spend with vendor fees
    #[builder(default, setter(into))]
    pub budget_type: Option<BudgetType>,

    /// Supported revenue types: CPM, CPC, CPCV, CPI, CPA
    #[builder(default, setter(into))]
    pub revenue_type: Option<RevenueType>,

    /// If a revenue_type is set, this is field is the basis of calculation. For example, if
    /// revenue_type is CPM and revenue_amount is 5.12, revenue will be calculated as a $5.12 CPM.
    #[builder(default, setter(into))]
    pub revenue_amount: Option<f64>,

    // This field is deprecated. Pacing is available at the Line Item level.
    // pub pacing: Option<u64>,
    /// ID of a Bid Modifier object to associate with the Campaign. If set, max_bid must also be
    /// set.
    #[builder(default, setter(into))]
    pub bid_modifier_id: Option<u64>,

    /// ID of the Delivery Modifier to associate with Line Items under this Campaign
    #[builder(default, setter(into))]
    pub delivery_modifier_id: Option<u64>,

    /// Maximum bid after taking into consideration any Bid Modifiers.
    #[builder(default, setter(into))]
    pub max_bid: Option<f64>,

    /// Start date of the Campaign. No Line Items associated with the Campaign can have start dates
    /// prior to this date.
    #[builder(default, setter(into))]
    pub start_date: String,

    /// End date of the Campaign. No Line Items associated with the Campaign can have end dates
    /// after this date. End date must be provided in order to pace.
    #[builder(default, setter(into))]
    pub end_date: Option<String>,

    /// Frequency cap JSON.
    #[builder(default, setter(into))]
    pub frequency_cap: Option<Vec<FrequencyCap>>,

    /// The method of frequency capping. All Line Items must match Campaign-level if set. For
    /// definitions, see the Frequency Cap guide.
    #[builder(default, setter(into))]
    pub frequency_cap_type: Option<FrequencyCapType>,

    /// Continents in which the Campaign is eligible to serve. Inherited from the Advertiser object
    /// if left blank.
    #[builder(default, setter(into))]
    pub continents: Option<Vec<Continent>>,

    /// Currency in which all Line Items under this Campaign will bid. Cannot be changed once set.
    /// If a default is set at the Advertiser level, it will be inherited here.
    #[builder(default, setter(into))]
    pub currency: Option<Currency>,

    /// An alternative id to lookup the Campaign, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,

    /// Notes, up to 255 chars
    #[builder(default, setter(into))]
    pub notes: Option<String>,

    /// Is it active?
    #[builder(default)]
    pub active: bool,
}

impl Create<Campaign> for CreateCampaign {
    fn into_resource(self, campaign_id: u64) -> Campaign {
        Campaign {
            campaign_id,
            advertiser_id: self.advertiser_id,
            campaign_name: self.campaign_name,
            campaign_budget: self.campaign_budget,
            daily_budget: self.daily_budget,
            budget_type: self.budget_type,
            revenue_type: self.revenue_type,
            revenue_amount: self.revenue_amount,
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

#[derive(Clone, Debug, Serialize, TypedBuilder)]
pub struct DeleteCampaign {
    #[builder(default)]
    campaign_id: u64,
}

impl Delete<Campaign> for DeleteCampaign {}

impl Delete<Campaign> for Campaign {}
