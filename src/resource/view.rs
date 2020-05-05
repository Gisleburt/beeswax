//! The View API method allows you to flexibly GET data from the Buzz system. It is especially
//! useful for querying static tables to discover ad types. acceptable mime types, or other lookup
//! fields. Views must be created by the Buzz administrator. Only GET requests are supported.

use crate::resource::{common::ViewName, Read, Resource};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::ops::Deref;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct View(JsonValue);

impl Resource for View {
    const NAME: &'static str = "view";
}

impl Deref for View {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The View List API method allows you to GET metadata about how a View and the fields within that
/// View should be displayed in a UI. There is no write API for this data, it must be manually
/// populated in SQL by an administrator. The primary use case for this API call is to display the
/// data from a view in a user interface. Only GET requests are supported.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ViewList(JsonValue);

impl Resource for ViewList {
    const NAME: &'static str = "view_list";
}

impl Deref for ViewList {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Serialize)]
pub enum ViewName {
    /// Categorization for advertisers (e.g. automotive)
    #[serde(rename = "advertiser_category")]
    AdvertiserCategory,
    /// A master list of all advertisers
    #[serde(rename = "advertiser_index")]
    AdvertiserIndex,
    /// List of sensitive categories to describe the advertiser (e.g. alcohol)
    #[serde(rename = "advertiser_sensitive_category")]
    AdvertiserSensitiveCategory,
    /// Advertiser objects including many fields showing updated performance results
    #[serde(rename = "advertiser_stats")]
    AdvertiserStats,
    /// Codes for user bandwidth targeting
    #[serde(rename = "bandwidth")]
    Bandwidth,
    /// List of supported bidding strategies
    #[serde(rename = "bidding_strategies")]
    BiddingStrategies,
    /// Campaign objects including many fields showing updated performance results
    #[serde(rename = "campaign_stats")]
    CampaignStats,
    /// Global list of cities and regions. Use cities_active_view view for targeting.
    #[serde(rename = "cities")]
    Cities,
    /// List of cities and regions that are enabled for targeting (subset based on countries_active
    /// view, below)
    #[serde(rename = "cities_active_view")]
    CitiesActiveView,
    /// A list of continents in which data centers exist. Used to determine where a Campaign may be
    /// eligible to serve or where a Segment Upload is uploaded.
    #[serde(rename = "continents")]
    Continents,
    /// List of all country codes. Use countries_active_view view for targeting.
    #[serde(rename = "countries")]
    Countries,
    /// List of country codes that are enabled for targeting
    #[serde(rename = "countries_active_view")]
    CountriesActiveView,
    /// Creative approval vendors
    #[serde(rename = "creative_approval_vendors")]
    CreativeApprovalVendors,
    /// Acceptable creative sizes (width and height)
    #[serde(rename = "creative_sizes")]
    CreativeSizes,
    /// Acceptable currency codes
    #[serde(rename = "currency")]
    Currency,
    /// Google Metro codes (similar to DMAs)
    #[serde(rename = "DMA")]
    Dma,
    /// Environments in which the ad may be served. 0=web, 1=in-app
    #[serde(rename = "environment_types")]
    EnvironmentTypes,
    /// Event types
    #[serde(rename = "event_types")]
    EventTypes,
    /// Types of tags supported for events (event_tag API) and segments (segment_tag API)
    #[serde(rename = "event_tag_types")]
    EventTagTypes,
    /// Inventory sources (e.g. Google)
    #[serde(rename = "inventory_source")]
    InventorySource,
    /// Line item objects including many fields showing updated performance results
    #[serde(rename = "line_item_stats")]
    LineItemStats,
    /// Line Item types (e.g. banner, mobile, video)
    #[serde(rename = "line_item_types")]
    LineItemTypes,
    /// Useful lookup data about the 100,000 most commonly seen mobile apps
    #[serde(rename = "lookup_apps")]
    LookupApps,
    /// The 100,000 most commonly seen domains
    #[serde(rename = "lookup_domains")]
    LookupDomains,
    /// The 100,000 most commonly seen combinations of site_id and placement_id
    #[serde(rename = "lookup_inventory")]
    LookupInventory,
    /// Ad server macros allowable in Creatives and Creative Templates
    #[serde(rename = "macros")]
    Macros,
    /// Acceptable mime types for Creative Asset upload
    #[serde(rename = "mime_types")]
    MimeTypes,
    /// Object types (e.g. user, creative, targeting_templates)
    #[serde(rename = "object_types")]
    ObjectTypes,
    /// Global list of regions, including US States. Use regions_active_view for targeting.
    #[serde(rename = "regions")]
    Regions,
    /// List of regions enabled for targeting.
    #[serde(rename = "regions_active_view")]
    RegionsActiveView,
    /// Fields in reporting that are calculated when a reporting request is submitted
    #[serde(rename = "report_calculated_fields")]
    ReportCalculatedFields,
    /// List of special reporting date parameters supported such as today
    #[serde(rename = "report_date_filters")]
    ReportDateFilters,
    /// List of fields per report (see reports below)
    #[serde(rename = "report_fields")]
    ReportFields,
    /// List of supported reports
    #[serde(rename = "reports")]
    Reports,
    /// Acceptable formats for uploading segment files
    #[serde(rename = "segment_file_format")]
    SegmentFileFormat,
    /// List of segments with estimated number of users in each segment per continent
    #[serde(rename = "segment_stats")]
    SegmentStats,
    /// List of possible statuses for a segment file upload
    #[serde(rename = "segment_upload_status")]
    SegmentUploadStatus,
    /// List of Banner Ad Types from OpenRTB 5.2
    #[serde(rename = "tag_types")]
    TagTypes,
    /// Types of video APIs supported
    #[serde(rename = "video_apis")]
    VideoApis,
    /// Types of video companions (static, HTML, etc)
    #[serde(rename = "video_companion_types")]
    VideoCompanionTypes,
    /// List of videos waiting to be transcoded
    #[serde(rename = "video_creative_transcoding_queue")]
    VideoCreativeTranscodingQueue,
    /// List of transcoding profiles supported
    #[serde(rename = "video_encoding_profiles")]
    VideoEncodingProfiles,
    /// Types of video protocols supported
    #[serde(rename = "video_protocols")]
    VideoProtocols,
}

#[derive(Clone, Serialize)]
pub struct ReadView {
    view_name: ViewName,
}

impl Read<View> for ReadView {}

#[derive(Clone, Serialize)]
pub struct ReadViewList {
    view_name: ViewName,
}

impl Read<ViewList> for ReadViewList {}
