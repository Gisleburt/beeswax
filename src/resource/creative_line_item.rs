use crate::resource::{Create, Delete, Find, Resource};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct CreativeLineItem {
    cli_id: u64,
    /// Unique ID of the Creative to be associated. Must bethe same type as the Line Item.
    creative_id: u64,
    /// Unique D of the Line Item to be associated. Must be the same type as the Creative.
    line_item_id: u64,
    /// When the Line Item has creative_weighting_method set to WEIGHTED, this field represents the
    /// desired ratio of delivery against all other associated Creatives of the same size and type.
    /// Should be an integer between 1 and 100.
    weighting: Option<u64>,
    /// Start date for the Creative to serve within this Line Item, optional. If either start_date
    /// or end_date is set, both values will override the Creative level dates, otherwise Creative
    /// level will be used.
    start_date: Option<String>,
    /// End date for the Creative to serve within this Line Item, optional. If either start_date or
    /// end_date is set, both values will override the Creative level dates, otherwise Creative
    /// level will be used.
    end_date: Option<String>,
    /// Is it active?
    active: bool,
    account_id: u64,
    create_date: String,
    update_date: String,
    push_status: u64,
    push_update: bool,
    buzz_key: String,
}

impl Resource for CreativeLineItem {
    const NAME: &'static str = "creative_line_item";
}

#[derive(Clone, Default, Serialize)]
struct FindCreativeLineItem {
    /// Unique ID of the Creative Line Item association
    cli_id: Option<u64>,
    /// Unique ID of the Creative.
    creative_id: Option<u64>,
    /// Unique D of the Line Item.
    line_item_id: Option<u64>,
    /// Start date for the Creative to serve within this Line Item. If either start_date or end_date is set, both values will override the Creative level dates, otherwise Creative level will be used.
    start_date: Option<String>,
    /// End date for the Creative to serve within this Line Item. If either start_date or end_date is set, both values will override the Creative level dates, otherwise Creative level will be used.
    end_date: Option<String>,
    /// Is it active?
    active: Option<bool>,
}

impl Find<CreativeLineItem> for CreativeLineItem {}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CreateCreativeLineItem {
    /// Unique ID of the Creative to be associated. Must bethe same type as the Line Item.
    creative_id: u64,
    /// Unique D of the Line Item to be associated. Must be the same type as the Creative.
    line_item_id: u64,
    /// When the Line Item has creative_weighting_method set to WEIGHTED, this field represents the
    /// desired ratio of delivery against all other associated Creatives of the same size and type.
    /// Should be an integer between 1 and 100.
    weighting: Option<u64>,
    /// Start date for the Creative to serve within this Line Item, optional. If either start_date
    /// or end_date is set, both values will override the Creative level dates, otherwise Creative
    /// level will be used.
    start_date: Option<String>,
    /// End date for the Creative to serve within this Line Item, optional. If either start_date or
    /// end_date is set, both values will override the Creative level dates, otherwise Creative
    /// level will be used.
    end_date: Option<String>,
    /// Is it active?
    active: bool,
}

impl Create<CreativeLineItem> for CreateCreativeLineItem {
    fn into_resource(self, cli_id: u64) -> CreativeLineItem {
        CreativeLineItem {
            cli_id,
            creative_id: self.creative_id,
            line_item_id: self.line_item_id,
            weighting: self.weighting,
            start_date: self.start_date,
            end_date: self.end_date,
            active: self.active,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct DeleteCreativeLineItem {
    cli_id: u64,
}

impl Delete<CreativeLineItem> for DeleteCreativeLineItem {}

impl Delete<CreativeLineItem> for CreativeLineItem {}
