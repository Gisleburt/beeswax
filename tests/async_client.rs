extern crate beeswax;

mod helper;

use beeswax::resource::Advertiser;
use helper::{get_async_beeswax_client, random_string};

#[tokio::test]
async fn async_client() {
    let beeswax_client = get_async_beeswax_client()
        .await
        .expect("could not get beeswax client");
    let create_advertiser = Advertiser::create_builder()
        .advertiser_name(random_string("Advertiser Name"))
        .build();
    let created_advertiser = beeswax_client
        .create(&create_advertiser)
        .await
        .expect("Could not create");

    let read_advertiser = Advertiser::read_builder()
        .advertiser_id(created_advertiser.advertiser_id)
        .build();

    let mut advertiser = beeswax_client
        .read(&read_advertiser)
        .await
        .expect("Could not read")
        .pop()
        .expect("No advertiser found");

    advertiser.advertiser_name = random_string("Updated advertiser");

    beeswax_client
        .update(&advertiser)
        .await
        .expect("Could not update");

    beeswax_client
        .delete(&advertiser)
        .await
        .expect("Could not delete")
}
