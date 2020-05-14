extern crate beeswax;

use crate::helper::random_string;
use beeswax::resource::campaign::ReadCampaign;
use beeswax::resource::{
    advertiser::CreateAdvertiser, campaign::CreateCampaign, common::BudgetType,
    common::FrequencyCap, common::FrequencyCapType, common::RevenueType,
};

mod helper;

#[tokio::test]
async fn test_campaign() {
    let beeswax_client = helper::get_beeswax_client().await.unwrap();

    let create_advertiser = CreateAdvertiser {
        advertiser_name: random_string("Advertiser name"),
        ..Default::default()
    };

    let advertiser = beeswax_client.create(&create_advertiser).await.unwrap();

    let create_campaign = CreateCampaign {
        advertiser_id: advertiser.advertiser_id,
        campaign_name: random_string("Campaign name"),
        campaign_budget: 1.0,
        daily_budget: Some(1.0),
        budget_type: Some(BudgetType::Impressions),
        revenue_type: Some(RevenueType::CPA),
        revenue_amount: Some(0.0),
        bid_modifier_id: None,
        delivery_modifier_id: None,
        max_bid: None,
        start_date: "2020-01-01".to_string(),
        end_date: Some("2037-01-01".to_string()),
        frequency_cap: Some(vec![
            FrequencyCap {
                duration: Some(60),
                impressions: Some(10),
            },
            FrequencyCap {
                duration: Some(2592000),
                impressions: Some(1000),
            },
        ]),
        frequency_cap_type: Some(FrequencyCapType::Standard),
        continents: None,
        currency: None,
        alternative_id: None,
        notes: None,
        active: false,
    };

    let campaign = beeswax_client.create(&create_campaign).await.unwrap();

    let read_campaign = ReadCampaign {
        campaign_id: Some(campaign.campaign_id),
        // advertiser_id: Some(campaign.advertiser_id),
        // campaign_name: Some(campaign.campaign_name.clone()),
        // bid_modifier_id: campaign.bid_modifier_id,
        // delivery_modifier_id: campaign.delivery_modifier_id,
        // alternative_id: campaign.alternative_id.clone(),
        // active: Some(campaign.active),
        ..Default::default()
    };

    let mut read_campaign = beeswax_client
        .read(&read_campaign)
        .await
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(read_campaign.campaign_id, campaign.campaign_id);
    assert_eq!(read_campaign.advertiser_id, campaign.advertiser_id);
    assert_eq!(read_campaign.campaign_name, campaign.campaign_name);
    assert_eq!(read_campaign.campaign_budget, campaign.campaign_budget);
    assert_eq!(read_campaign.daily_budget, campaign.daily_budget);
    assert_eq!(read_campaign.budget_type, campaign.budget_type);
    assert_eq!(read_campaign.revenue_type, campaign.revenue_type);
    assert_eq!(read_campaign.revenue_amount, campaign.revenue_amount);
    assert_eq!(read_campaign.bid_modifier_id, campaign.bid_modifier_id);
    assert_eq!(read_campaign.delivery_modifier_id, campaign.delivery_modifier_id);
    assert_eq!(read_campaign.max_bid, campaign.max_bid);
    // assert_eq!(read_campaign.start_date, campaign.start_date);
    // assert_eq!(read_campaign.end_date, campaign.end_date);
    assert_eq!(read_campaign.frequency_cap, campaign.frequency_cap);
    assert_eq!(read_campaign.frequency_cap_type, campaign.frequency_cap_type);
    // assert_eq!(read_campaign.continents, Some(vec![advertiser.default_continent.unwrap()]));
    // assert_eq!(read_campaign.currency, advertiser.default_currency);
    assert_eq!(read_campaign.alternative_id, campaign.alternative_id);
    assert_eq!(read_campaign.notes, campaign.notes);
    assert_eq!(read_campaign.active, campaign.active);

    read_campaign.campaign_name = random_string("Updated Campaign Name");

    beeswax_client.update(&read_campaign).await.unwrap();

    let find_campaign = ReadCampaign {
        campaign_id: Some(campaign.campaign_id),
        ..Default::default()
    };

    let updated_advertiser = beeswax_client
        .read(&find_campaign)
        .await
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(updated_advertiser.campaign_name, read_campaign.campaign_name);

}
