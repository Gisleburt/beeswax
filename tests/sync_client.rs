extern crate beeswax;

mod helper;

use beeswax::resource::Advertiser;
use helper::{get_sync_beeswax_client, random_string};

#[test]
fn sync_client() {
    let beeswax_client = get_sync_beeswax_client()
        .expect("could not get beeswax client");
    let create_advertiser = Advertiser::create_builder()
        .advertiser_name(random_string("Advertiser Name"))
        .build();
    let created_advertiser = beeswax_client
        .create(&create_advertiser)
        .expect("Could not create");

    let read_advertiser = Advertiser::read_builder()
        .advertiser_id(created_advertiser.advertiser_id)
        .build();

    let mut advertiser = beeswax_client
        .read(&read_advertiser)
        .expect("Could not read")
        .pop()
        .expect("No advertiser found");

    advertiser.advertiser_name = random_string("Updated advertiser");

    beeswax_client
        .update(&advertiser)
        .expect("Could not update");

    beeswax_client
        .delete(&advertiser)
        .expect("Could not delete")
}
