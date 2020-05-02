use crate::resource::{Create, Delete, Read, Resource};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Every Campaign, Line Item, and Creatives belongs to an Advertiser. Advertisers are typically the
/// entity paying the bills for the ads that run. See: Advertisers, Campaigns, Line_Items,
/// Creatives.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Advertiser {
    /// Unique ID of the advertiser
    pub advertiser_id: u64,
    /// Unique name for the advertiser, must be unique per account.
    pub advertiser_name: String,
    /// Creative attributes in JSON
    ///
    /// <module_name> `object` The overall statement can include any number of modules, but each
    /// module may only appear once. Example of a module is advertiser
    ///
    /// <key> `array of mixed types` Key must belong to the module selected, the value must be a
    /// list of one or more values corresponding to the key. Values may be integers or strings
    /// depending on the key.
    pub attributes: Option<JsonValue>,
    /// The conversion attribution to use for Events owned by this Advertiser. Must be a valid
    /// attribution method as found in the conversion_atrtibution_methods view. At this time only
    /// method 1 (last click) is supported. Once a conversion method is chosen, it cannot be
    /// changed.
    pub conversion_method_id: Option<u64>,
    /// Click URL to use by default for objects created under this advertiser
    pub default_click_url: Option<String>,
    /// Continent in which this Advertiser's Campaigns should be eligible to serve. Can be changed
    /// at the Campaign level.
    pub default_continent: Option<String>,
    /// Currency to use as default for all Campaigns under this Advertiser
    pub default_currency: Option<String>,
    /// URL of a thumbnail image to use when a Creative does not have one. This is useful since some
    /// exchanges may not allow a Creative to run without a thumbnail.
    pub default_creative_thumbnail_url: Option<String>,
    /// The preset to use by default for all campaigns created under this advertiser. Note, presets
    /// can only be created in the UI.
    pub default_campaign_preset_id: Option<u64>,
    /// The preset to use by default for all line items created under this advertiser. Note, presets
    /// can only be created in the UI.
    pub default_line_item_preset_id: Option<u64>,
    /// An alternative id to associate, if desired
    pub alternative_id: Option<String>,
    /// Any notes desired, less than 255 chars
    pub notes: Option<String>,
    /// Is the advertier active
    pub active: Option<bool>,
    // Undocumented
    pub account_id: Option<u64>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub buzz_key: Option<String>,
}

impl Resource for Advertiser {
    const NAME: &'static str = "advertiser";
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReadAdvertiser {
    /// Unique ID of the advertiser
    pub advertiser_id: Option<u64>,
    /// An alternative id to lookup the object, if desired
    pub alternative_id: Option<String>,
    /// Unique name for the advertiser. Supports %LIKE% syntax
    pub advertiser_name: Option<String>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
}

impl Read<Advertiser> for ReadAdvertiser {}

/// Every Campaign, Line Item, and Creatives belongs to an Advertiser. Advertisers are typically the
/// entity paying the bills for the ads that run. See: Advertisers, Campaigns, Line_Items,
/// Creatives.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct CreateAdvertiser {
    /// Unique name for the advertiser, must be unique per account.
    pub advertiser_name: String,
    /// Creative attributes in JSON
    ///
    /// <module_name> `object` The overall statement can include any number of modules, but each
    /// module may only appear once. Example of a module is advertiser
    ///
    /// <key> `array of mixed types` Key must belong to the module selected, the value must be a
    /// list of one or more values corresponding to the key. Values may be integers or strings
    /// depending on the key.
    pub attributes: Option<JsonValue>,
    /// The conversion attribution to use for Events owned by this Advertiser. Must be a valid
    /// attribution method as found in the conversion_atrtibution_methods view. At this time only
    /// method 1 (last click) is supported. Once a conversion method is chosen, it cannot be
    /// changed.
    pub conversion_method_id: Option<u64>,
    /// Click URL to use by default for objects created under this advertiser
    pub default_click_url: Option<String>,
    /// Continent in which this Advertiser's Campaigns should be eligible to serve. Can be changed
    /// at the Campaign level.
    pub default_continent: Option<String>,
    /// Currency to use as default for all Campaigns under this Advertiser
    pub default_currency: Option<String>,
    /// URL of a thumbnail image to use when a Creative does not have one. This is useful since some
    /// exchanges may not allow a Creative to run without a thumbnail.
    pub default_creative_thumbnail_url: Option<String>,
    /// The preset to use by default for all campaigns created under this advertiser. Note, presets
    /// can only be created in the UI.
    pub default_campaign_preset_id: Option<u64>,
    /// The preset to use by default for all line items created under this advertiser. Note, presets
    /// can only be created in the UI.
    pub default_line_item_preset_id: Option<u64>,
    /// An alternative id to associate, if desired
    pub alternative_id: Option<String>,
    /// Any notes desired, less than 255 chars
    pub notes: Option<String>,
    /// Is the advertier active
    pub active: Option<bool>,
}

impl Create<Advertiser> for CreateAdvertiser {
    fn into_resource(self, advertiser_id: u64) -> Advertiser {
        Advertiser {
            advertiser_id,
            advertiser_name: self.advertiser_name,
            attributes: self.attributes,
            conversion_method_id: self.conversion_method_id,
            default_click_url: self.default_click_url,
            default_continent: self.default_continent,
            default_currency: self.default_currency,
            default_creative_thumbnail_url: self.default_creative_thumbnail_url,
            default_campaign_preset_id: self.default_campaign_preset_id,
            default_line_item_preset_id: self.default_line_item_preset_id,
            alternative_id: self.alternative_id,
            notes: self.notes,
            active: self.active,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct DeleteAdvertiser {
    advertiser_id: u64,
}

impl Delete<Advertiser> for DeleteAdvertiser {}

impl Delete<Advertiser> for Advertiser {}
