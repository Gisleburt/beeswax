//! A Creative object defines the payload to be delivered to the end user. Creatives belong to
//! Advertisers and are associated with Line Items. The Creative object has interactions with
//! Creative Templates and Creative Assets / Video Assets as described in Creatives, Creative
//! Assets, Templates, Rules.

use crate::resource::{
    advertiser::Advertiser, common::CreativeType, Create, Delete, Read, Resource,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Creative {
    /// Unique ID of the Creative
    pub creative_id: u64,

    /// Must belong to the same account as the Advertiser and be active
    pub advertiser_id: u64,

    /// Name of the Creative, e.g. "Blue Banner Ad"
    pub creative_name: String,

    /// The type of creative. 0=banner, 1=video, 2=native
    pub creative_type: CreativeType,

    /// Width in pixels. Use the creative_sizes view to see all acceptable width and height
    /// combinations.
    pub width: Option<u64>,

    /// Height in pixels. Use the creative_sizes view to see all acceptable width and height
    /// combinations.
    pub height: Option<u64>,

    /// Is the creative sizeless, meaning it can match any size placement that is an interstitial.
    /// Native creatives should also be marked as sizeless.
    pub sizeless: Option<bool>,

    /// Is the creative intended to serve in a secure (HTTPS) environment.
    pub secure: bool,

    /// URL the ad should click or tap to, must be a valid URL. This field is required when using a
    /// Creative Template that is not a tag (e.g. an image or video)
    pub click_url: Option<String>,

    /// DEPRECATED, use primary_asset and secondary_asset fields instead.
    // pub creative_assets: Option<Vec<u64>>,

    /// ID of the creative_asset to use in the Creative, for example the ID of an Image or Video.
    pub primary_asset: Option<u64>,

    /// ID of a secondary creative_asset to use in the Creative. Most commonly used for the
    /// companion asset within a Video Creative
    pub secondary_asset: Option<u64>,

    /// For Native creatives, the NativeOffer to be used for the Creative content
    pub native_offer: Option<u64>,

    /// A JSON representation of the fields required by the Creative Template, validated against the
    /// Creative Template. Schema of json varies.
    pub creative_content: Option<JsonValue>,

    /// For tag creatives the tag can be placed in this field and on save the creative_rule_key will
    /// be applied to insert relevant macros. This is recommended vs completing the creative_content
    /// field directly.
    pub creative_content_tag: Option<String>,

    /// The ID of the Creative Template to use for this creative. Must be a valid and active
    /// Creative Template that either belongs to this Account, OR is marked as "global".
    pub creative_template_id: u64,

    /// DEPRECATED
    pub creative_rule_id: Option<u64>,

    /// The key corresponding to the creative_rule to apply to the creative_content_tag field. The
    /// rule will insert click and timestamp macros. You can use auto_detect for easiest
    /// implementation.
    pub creative_rule_key: Option<String>,

    /// Creative Attributes JSON.
    pub attributes: Option<JsonValue>,

    /// List of URLs to be added to the Creative as pixels
    pub pixels: Option<Vec<String>>,

    /// List of VAST tracking events and associated URLs to allow third party tracking of video
    /// events
    pub events: Option<JsonValue>,

    /// List of objects to track VAST video progress
    pub progress_events: Option<JsonValue>,

    /// List of CreativeAddOn IDs to add to the Creative
    pub creative_addons: Option<Vec<u64>>,

    /// URL to an image thumbnail for the creative. This field will be automatically set if you
    /// associate the creative with a Creative Asset that has a valid thumbnail but must be updated
    /// manually when using a tag-based Creative. Thumbnail is required by some exchanges to serve.
    pub creative_thumbnail_url: Option<String>,

    /// Start date for the creative, optional
    pub start_date: Option<String>,

    /// End date for the creative, optional
    pub end_date: Option<String>,

    /// An alternative id to lookup the Creative, if desired
    pub alternative_id: Option<String>,

    /// Notes about the Creative, up to 255 chars
    pub notes: Option<String>,

    /// Is the Creative active?
    pub active: Option<bool>,
    // Undocumented
    pub creative_status_id: Option<bool>,
    pub creative_attributes: JsonValue,
    // ToDo: Unknown type,
    // pub creative_assets: Unknown,
    pub creative_content_munge: Option<String>,
    pub preview_token: Option<String>,
    // ToDo: Unknown type,
    // pub frequency_cap: Unknown,
    // ToDo: Unknown type,
    // pub scripts: Unknown,
    pub push_status: Option<u64>,
    pub push_update: Option<bool>,
    pub account_id: Option<u64>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
    pub buzz_key: Option<String>,
}

impl Creative {
    /// Create a builder for CreateCreative
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Creative;
    ///
    /// let create_creative = Creative::create_builder()
    ///     .creative_name("Some name")
    ///     .advertiser_id(1)
    ///     .build();
    ///
    /// let creative = beeswax_client.create(&create_creative).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_builder() -> CreateCreativeBuilder<(
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
        (),
        (),
    )> {
        CreateCreative::builder()
    }

    /// Create a builder for ReadCreative
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Creative;
    ///
    /// let read_creative = Creative::read_builder()
    ///     .creative_name("Some name".to_string())
    ///     .build();
    ///
    /// let creatives = beeswax_client.read(&read_creative).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_builder() -> ReadCreativeBuilder<((), (), (), (), (), (), (), (), ())> {
        ReadCreative::builder()
    }

    /// Create a builder for DeleteCreative
    /// ```
    /// # use std::error::Error;
    /// # use beeswax::client::async_client::AsyncInMemoryClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// # let mut beeswax_client = AsyncInMemoryClient::new();
    /// use beeswax::resource::Creative;
    ///
    /// let delete_creative = Creative::delete_builder()
    ///     .creative_id(10)
    ///     .build();
    ///
    /// beeswax_client.delete(&delete_creative).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_builder() -> DeleteCreativeBuilder<((),)> {
        DeleteCreative::builder()
    }
}

impl Resource for Creative {
    const NAME: &'static str = "creative";
}

#[derive(Clone, Default, Serialize, TypedBuilder)]
pub struct ReadCreative {
    /// Unique ID of the Creative
    #[builder(default, setter(into))]
    pub creative_id: Option<u64>,
    /// Must belong to the same account as the Advertiser
    #[builder(default, setter(into))]
    pub advertiser_id: Option<u64>,
    /// Name of the Creative, e.g. "Blue Banner Ad"
    #[builder(default, setter(into))]
    pub creative_name: Option<String>,
    /// ID for the type of creative. 0=banner, 1=video, 2=native, etc.
    #[builder(default, setter(into))]
    pub creative_type: Option<CreativeType>,
    /// The ID of the Creative Template to use for this creative. Must be a valid and active Creative Template that either belongs to this Account, OR is marked as "global".
    #[builder(default, setter(into))]
    pub creative_template_id: Option<u64>,
    /// An alternative id to lookup the creative, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,
    /// Is the Creative active?
    #[builder(default, setter(into))]
    pub active: Option<bool>,
    #[builder(default, setter(into))]
    pub create_date: Option<String>,
    #[builder(default, setter(into))]
    pub update_date: Option<String>,
}

impl Read<Creative> for ReadCreative {}

impl PartialEq<Creative> for ReadCreative {
    fn eq(&self, other: &Creative) -> bool {
        (self.creative_id.is_none() || self.creative_id == Some(other.creative_id))
            && (self.advertiser_id.is_none() || self.advertiser_id == Some(other.advertiser_id))
            && (self.creative_name.is_none()
                || self.creative_name.as_ref() == Some(&other.creative_name))
            && (self.creative_type.is_none() || self.creative_type == Some(other.creative_type))
            && (self.creative_template_id.is_none()
                || self.creative_template_id == Some(other.creative_template_id))
            && (self.alternative_id.is_none() || self.alternative_id == other.alternative_id)
            && (self.active.is_none() || self.active == other.active)
            && (self.create_date.is_none() || self.create_date == other.create_date)
            && (self.update_date.is_none() || self.update_date == other.update_date)
    }
}

/// Create a search criteria for creatives for the given advertiser
impl From<Advertiser> for ReadCreative {
    fn from(adveritiser: Advertiser) -> Self {
        ReadCreative {
            advertiser_id: Some(adveritiser.advertiser_id),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCreative {
    /// Must belong to the same account as the Advertiser and be active
    pub advertiser_id: u64,

    /// Name of the Creative, e.g. "Blue Banner Ad"
    #[builder(setter(into))]
    pub creative_name: String,

    /// The type of creative. 0=banner, 1=video, 2=native
    #[builder(default, setter(into))]
    pub creative_type: CreativeType,

    /// Width in pixels. Use the creative_sizes view to see all acceptable width and height
    /// combinations.
    #[builder(default, setter(into))]
    pub width: Option<u64>,

    /// Height in pixels. Use the creative_sizes view to see all acceptable width and height
    /// combinations.
    #[builder(default, setter(into))]
    pub height: Option<u64>,

    /// Is the creative sizeless, meaning it can match any size placement that is an interstitial.
    /// Native creatives should also be marked as sizeless.
    #[builder(default, setter(into))]
    pub sizeless: Option<bool>,

    /// Is the creative intended to serve in a secure (HTTPS) environment.
    #[builder(default, setter(into))]
    pub secure: bool,

    /// URL the ad should click or tap to, must be a valid URL. This field is required when using a
    /// Creative Template that is not a tag (e.g. an image or video)
    #[builder(default, setter(into))]
    pub click_url: Option<String>,

    /// DEPRECATED, use primary_asset and secondary_asset fields instead.
    // pub creative_assets: Option<Vec<u64>>,

    /// ID of the creative_asset to use in the Creative, for example the ID of an Image or Video.
    #[builder(default, setter(into))]
    pub primary_asset: Option<u64>,

    /// ID of a secondary creative_asset to use in the Creative. Most commonly used for the
    #[builder(default, setter(into))]
    /// companion asset within a Video Creative
    pub secondary_asset: Option<u64>,

    /// For Native creatives, the NativeOffer to be used for the Creative content
    #[builder(default, setter(into))]
    pub native_offer: Option<u64>,

    /// A JSON representation of the fields required by the Creative Template, validated against the
    /// Creative Template. Schema of json varies.
    #[builder(default, setter(into))]
    pub creative_content: Option<JsonValue>,

    /// For tag creatives the tag can be placed in this field and on save the creative_rule_key will
    /// be applied to insert relevant macros. This is recommended vs completing the creative_content
    /// field directly.
    #[builder(default, setter(into))]
    pub creative_content_tag: Option<String>,

    /// The ID of the Creative Template to use for this creative. Must be a valid and active
    /// Creative Template that either belongs to this Account, OR is marked as "global".
    #[builder(default, setter(into))]
    pub creative_template_id: u64,

    /// DEPRECATED
    // pub creative_rule_id: Option<u64>,

    /// The key corresponding to the creative_rule to apply to the creative_content_tag field. The
    /// rule will insert click and timestamp macros. You can use auto_detect for easiest
    /// implementation.
    #[builder(default, setter(into))]
    pub creative_rule_key: Option<String>,

    /// Creative Attributes JSON.
    #[builder(default, setter(into))]
    pub attributes: Option<JsonValue>,

    /// List of URLs to be added to the Creative as pixels
    #[builder(default, setter(into))]
    pub pixels: Option<Vec<String>>,

    /// List of VAST tracking events and associated URLs to allow third party tracking of video
    /// events
    #[builder(default, setter(into))]
    pub events: Option<JsonValue>,

    /// List of objects to track VAST video progress
    #[builder(default, setter(into))]
    pub progress_events: Option<JsonValue>,

    /// List of CreativeAddOn IDs to add to the Creative
    #[builder(default, setter(into))]
    pub creative_addons: Option<Vec<u64>>,

    /// URL to an image thumbnail for the creative. This field will be automatically set if you
    /// associate the creative with a Creative Asset that has a valid thumbnail but must be updated
    /// manually when using a tag-based Creative. Thumbnail is required by some exchanges to serve.
    #[builder(default, setter(into))]
    pub creative_thumbnail_url: Option<String>,

    /// Start date for the creative, optional
    #[builder(default, setter(into))]
    pub start_date: Option<String>,

    /// End date for the creative, optional
    #[builder(default, setter(into))]
    pub end_date: Option<String>,

    /// An alternative id to lookup the Creative, if desired
    #[builder(default, setter(into))]
    pub alternative_id: Option<String>,

    /// Notes about the Creative, up to 255 chars
    #[builder(default, setter(into))]
    pub notes: Option<String>,

    /// Is the Creative active?
    #[builder(default, setter(into))]
    pub active: Option<bool>,
}

impl Create<Creative> for CreateCreative {
    fn into_resource(self, creative_id: u64) -> Creative {
        Creative {
            creative_id,
            advertiser_id: self.advertiser_id,
            creative_name: self.creative_name,
            creative_type: self.creative_type,
            width: self.width,
            height: self.height,
            sizeless: self.sizeless,
            secure: self.secure,
            click_url: self.click_url,
            // creative_assets: self.creative_assets,
            primary_asset: self.primary_asset,
            secondary_asset: self.secondary_asset,
            native_offer: self.native_offer,
            creative_content: self.creative_content,
            creative_content_tag: self.creative_content_tag,
            creative_template_id: self.creative_template_id,
            // creative_rule_id: self.creative_rule_id,
            creative_rule_key: self.creative_rule_key,
            attributes: self.attributes,
            pixels: self.pixels,
            events: self.events,
            progress_events: self.progress_events,
            creative_addons: self.creative_addons,
            creative_thumbnail_url: self.creative_thumbnail_url,
            start_date: self.start_date,
            end_date: self.end_date,
            alternative_id: self.alternative_id,
            notes: self.notes,
            active: self.active,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, TypedBuilder)]
pub struct DeleteCreative {
    creative_id: u64,
}

impl Delete<Creative> for DeleteCreative {}

impl Delete<Creative> for Creative {}
