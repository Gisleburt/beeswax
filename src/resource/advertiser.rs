use crate::resource::{Resource, SearchCriteria};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Advertiser {
    pub advertiser_id: u64,
    pub advertiser_name: String,
    // pub attributes: Unsure,
    pub default_click_url: Option<String>,
    pub default_continent: Option<String>,
    pub default_currency: Option<String>,
    pub default_creative_thumbnail_url: Option<String>,
    pub default_campaign_preset_id: Option<u64>,
    pub default_line_item_preset_id: Option<u64>,
    pub alternative_id: Option<String>,
    pub notes: Option<String>,
    pub active: Option<bool>,
}

impl Resource for Advertiser {
    const NAME: &'static str = "advertiser";
    const ID_FIELD: &'static str = "advertiser_id";
}

#[derive(Default, Serialize)]
pub struct AdvertiserSearchCriteria {
    pub advertiser_id: Option<u64>,
    pub alternative_id: Option<String>,
    pub advertiser_name: Option<String>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
}

impl SearchCriteria<Advertiser> for AdvertiserSearchCriteria {}
