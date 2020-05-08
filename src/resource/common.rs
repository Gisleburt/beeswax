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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Continent {
    #[serde(rename = "APAC")]
    AsiaPacific,
    #[serde(rename = "EMEA")]
    Europe,
    #[serde(rename = "NAM")]
    NorthAmerica,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Currency {
    #[serde(rename = "AUD")]
    AustralianDollar,
    #[serde(rename = "BRL")]
    BrazilianReal,
    #[serde(rename = "CAD")]
    CanadianDollar,
    #[serde(rename = "CHF")]
    SwissFranc,
    #[serde(rename = "EUR")]
    Euro,
    #[serde(rename = "GBP")]
    GreatBritishPound,
    #[serde(rename = "HKD")]
    HongKongDollar,
    #[serde(rename = "JPY")]
    JapaneseYen,
    #[serde(rename = "KRW")]
    SouthKoreanWon,
    #[serde(rename = "MXN")]
    MexicanPeso,
    #[serde(rename = "NZD")]
    NewZealandDollar,
    #[serde(rename = "SEK")]
    SwedishKrona,
    #[serde(rename = "SGD")]
    SingaporeDollar,
    #[serde(rename = "UAH")]
    UkrainianHryvnia,
    #[serde(rename = "USD")]
    UnitedStatesDollar,
}
