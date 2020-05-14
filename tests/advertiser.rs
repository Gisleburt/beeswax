extern crate beeswax;

mod helper;

use beeswax::resource::{
    advertiser::{CreateAdvertiser, ReadAdvertiser},
    common::{Continent, ConversionMethod, Currency},
};
use helper::{get_beeswax_client, random_string};

#[tokio::test]
async fn test_advertiser() {
    let beeswax_client = get_beeswax_client().await.unwrap();
    let create_advertiser = CreateAdvertiser {
        advertiser_name: random_string("Advertiser Name"),
        attributes: None,
        conversion_method_id: ConversionMethod::LastClick,
        default_click_url: Some("https://example.com".to_string()),
        default_continent: Some(Continent::NorthAmerica),
        default_currency: Some(Currency::UnitedStatesDollar),
        default_creative_thumbnail_url: Some("https://example.com/thumbnail.jpg".to_string()),
        default_campaign_preset_id: None,
        default_line_item_preset_id: None,
        alternative_id: Some(random_string("Alternative Id")),
        notes: Some(random_string("Notes")),
        active: false,
    };
    let advertiser = beeswax_client.create(&create_advertiser).await.unwrap();

    let find_advertiser = ReadAdvertiser {
        advertiser_id: Some(advertiser.advertiser_id),
        alternative_id: advertiser.alternative_id.clone(),
        advertiser_name: Some(advertiser.advertiser_name.clone()),
        create_date: None,
        update_date: None,
    };

    let mut read_advertiser = beeswax_client
        .read(&find_advertiser)
        .await
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(read_advertiser.advertiser_id, advertiser.advertiser_id);
    assert_eq!(read_advertiser.advertiser_name, advertiser.advertiser_name);
    // assert_eq!(read_advertiser.attributes, advertiser.attributes);
    assert_eq!(
        read_advertiser.conversion_method_id,
        advertiser.conversion_method_id
    );
    assert_eq!(
        read_advertiser.default_click_url,
        advertiser.default_click_url
    );
    assert_eq!(
        read_advertiser.default_continent,
        advertiser.default_continent
    );
    assert_eq!(
        read_advertiser.default_currency,
        advertiser.default_currency
    );
    assert_eq!(
        read_advertiser.default_creative_thumbnail_url,
        advertiser.default_creative_thumbnail_url
    );
    assert_eq!(
        read_advertiser.default_campaign_preset_id,
        advertiser.default_campaign_preset_id
    );
    assert_eq!(
        read_advertiser.default_line_item_preset_id,
        advertiser.default_line_item_preset_id
    );
    assert_eq!(read_advertiser.alternative_id, advertiser.alternative_id);
    assert_eq!(read_advertiser.notes, advertiser.notes);
    assert_eq!(read_advertiser.active, advertiser.active);

    read_advertiser.advertiser_name = random_string("Updated Advertiser Name");
    read_advertiser.default_click_url = Some("https://example.com/updated".to_string());
    read_advertiser.default_continent = Some(Continent::NorthAmerica);
    read_advertiser.default_currency = Some(Currency::UnitedStatesDollar);
    read_advertiser.default_creative_thumbnail_url =
        Some("https://example.com/updated/thumbnail.jpg".to_string());
    read_advertiser.alternative_id = Some(random_string("Updated Alternative Id"));
    read_advertiser.notes = Some("Updated Notes".to_string());
    read_advertiser.active = true;

    beeswax_client.update(&read_advertiser).await.unwrap();

    let find_advertiser = ReadAdvertiser {
        advertiser_id: Some(advertiser.advertiser_id),
        ..Default::default()
    };

    let updated_advertiser = beeswax_client
        .read(&find_advertiser)
        .await
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(
        updated_advertiser.advertiser_id,
        read_advertiser.advertiser_id
    );
    assert_eq!(
        updated_advertiser.advertiser_name,
        read_advertiser.advertiser_name
    );
    assert_eq!(updated_advertiser.attributes, read_advertiser.attributes);
    assert_eq!(
        updated_advertiser.conversion_method_id,
        read_advertiser.conversion_method_id
    );
    assert_eq!(
        updated_advertiser.default_click_url,
        read_advertiser.default_click_url
    );
    assert_eq!(
        updated_advertiser.default_continent,
        read_advertiser.default_continent
    );
    assert_eq!(
        updated_advertiser.default_currency,
        read_advertiser.default_currency
    );
    assert_eq!(
        updated_advertiser.default_creative_thumbnail_url,
        read_advertiser.default_creative_thumbnail_url
    );
    assert_eq!(
        updated_advertiser.default_campaign_preset_id,
        read_advertiser.default_campaign_preset_id
    );
    assert_eq!(
        updated_advertiser.default_line_item_preset_id,
        read_advertiser.default_line_item_preset_id
    );
    assert_eq!(
        updated_advertiser.alternative_id,
        read_advertiser.alternative_id
    );
    assert_eq!(updated_advertiser.notes, read_advertiser.notes);
    assert_eq!(updated_advertiser.active, read_advertiser.active);

    beeswax_client.delete(&updated_advertiser).await.unwrap();

    let results = beeswax_client.read(&find_advertiser).await.unwrap().len();

    assert_eq!(results, 0);
}
