pub struct Advertiser {
    advertiser_id: u64,
    advertiser_name: String,
    //attributes: Unsure
    default_click_url: String,
    default_continent: String,
    default_currency: String,
    default_creative_thumbnail_url: String,
    default_campaign_preset_id: i32,
    default_line_item_preset_id: i32,
    alternative_id: String,
    notes: String,
    active: bool,
}