//! Every Campaign, Line Item, and Creatives belongs to an Advertiser. Advertisers are typically the
//! entity paying the bills for the ads that run. See: Advertisers, Campaigns, Line_Items,
//! Creatives.

use crate::resource::{
    common::{Continent, ConversionMethod, Currency},
    Create, Delete, Read, Resource,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
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
    pub conversion_method_id: ConversionMethod,
    /// Click URL to use by default for objects created under this advertiser
    pub default_click_url: Option<String>,
    /// Continent in which this Advertiser's Campaigns should be eligible to serve. Can be changed
    /// at the Campaign level.
    pub default_continent: Option<Continent>,
    /// Currency to use as default for all Campaigns under this Advertiser
    pub default_currency: Option<Currency>,
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
    pub active: bool,
    // Undocumented
    pub account_id: Option<u64>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub buzz_key: Option<String>,
}

impl Advertiser {
    /// Create a builder for CreateAdvertiser
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Advertiser;
    ///
    /// let create_advertiser = Advertiser::create_builder()
    ///     .advertiser_name("Some name")
    ///     .build();
    ///
    /// let advertiser = beeswax_client.create(&create_advertiser).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_builder(
    ) -> CreateAdvertiserBuilder<((), (), (), (), (), (), (), (), (), (), (), ())> {
        CreateAdvertiser::builder()
    }

    /// Create a builder for ReadAdvertiser
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Advertiser;
    ///
    /// let read_advertiser = Advertiser::read_builder()
    ///     .advertiser_name("Some name".to_string())
    ///     .build();
    ///
    /// let advertisers = beeswax_client.read(&read_advertiser).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_builder() -> ReadAdvertiserBuilder<((), (), (), (), ())> {
        ReadAdvertiser::builder()
    }

    /// Create a builder for DeleteAdvertiser
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Advertiser;
    ///
    /// let delete_advertiser = Advertiser::delete_builder()
    ///     .advertiser_id(10)
    ///     .build();
    ///
    /// beeswax_client.delete(&delete_advertiser).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_builder() -> DeleteAdvertiserBuilder<((),)> {
        DeleteAdvertiser::builder()
    }
}

impl Resource for Advertiser {
    const NAME: &'static str = "advertiser";
}

#[derive(Clone, Debug, Default, Serialize, TypedBuilder)]
pub struct ReadAdvertiser {
    /// Unique ID of the advertiser
    #[builder(default, setter(into))]
    pub advertiser_id: Option<u64>,
    /// An alternative id to lookup the object, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,
    /// Unique name for the advertiser. Supports %LIKE% syntax
    #[builder(default, setter(into))]
    pub advertiser_name: Option<String>,
    #[builder(default, setter(into))]
    pub create_date: Option<String>,
    #[builder(default, setter(into))]
    pub update_date: Option<String>,
}

impl PartialEq<Advertiser> for ReadAdvertiser {
    fn eq(&self, other: &Advertiser) -> bool {
        (self.advertiser_id.is_none() || self.advertiser_id == Some(other.advertiser_id))
            && (self.alternative_id.is_none() || self.alternative_id == other.alternative_id)
            && (self.advertiser_name.is_none()
                || self.advertiser_name.as_ref() == Some(&other.advertiser_name))
            && (self.create_date.is_none() || self.create_date == other.create_date)
            && (self.update_date.is_none() || self.update_date == other.update_date)
    }
}

impl Read<Advertiser> for ReadAdvertiser {}

#[derive(Clone, Default, Debug, Deserialize, Serialize, TypedBuilder)]
pub struct CreateAdvertiser {
    /// Unique name for the advertiser, must be unique per account.
    #[builder(setter(into))]
    pub advertiser_name: String,
    /// Creative attributes in JSON
    ///
    /// <module_name> `object` The overall statement can include any number of modules, but each
    /// module may only appear once. Example of a module is advertiser
    ///
    /// <key> `array of mixed types` Key must belong to the module selected, the value must be a
    /// list of one or more values corresponding to the key. Values may be integers or strings
    /// depending on the key.
    #[builder(default)]
    pub attributes: Option<JsonValue>,
    /// The conversion attribution to use for Events owned by this Advertiser. Must be a valid
    /// attribution method as found in the conversion_atrtibution_methods view. At this time only
    /// method 1 (last click) is supported. Once a conversion method is chosen, it cannot be
    /// changed.
    #[builder(default)]
    pub conversion_method_id: ConversionMethod,
    /// Click URL to use by default for objects created under this advertiser
    #[builder(default)]
    pub default_click_url: Option<String>,
    /// Continent in which this Advertiser's Campaigns should be eligible to serve. Can be changed
    /// at the Campaign level.
    #[builder(default)]
    pub default_continent: Option<Continent>,
    /// Currency to use as default for all Campaigns under this Advertiser
    #[builder(default)]
    pub default_currency: Option<Currency>,
    /// URL of a thumbnail image to use when a Creative does not have one. This is useful since some
    /// exchanges may not allow a Creative to run without a thumbnail.
    #[builder(default)]
    pub default_creative_thumbnail_url: Option<String>,
    /// The preset to use by default for all campaigns created under this advertiser. Note, presets
    /// can only be created in the UI.
    #[builder(default)]
    pub default_campaign_preset_id: Option<u64>,
    /// The preset to use by default for all line items created under this advertiser. Note, presets
    /// can only be created in the UI.
    #[builder(default)]
    pub default_line_item_preset_id: Option<u64>,
    /// An alternative id to associate, if desired
    #[builder(default)]
    pub alternative_id: Option<String>,
    /// Any notes desired, less than 255 chars
    #[builder(default)]
    pub notes: Option<String>,
    /// Is the advertier active
    #[builder(default)]
    pub active: bool,
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

#[derive(Clone, Debug, Serialize, TypedBuilder)]
pub struct DeleteAdvertiser {
    advertiser_id: u64,
}

impl Delete<Advertiser> for DeleteAdvertiser {}

impl Delete<Advertiser> for Advertiser {}
