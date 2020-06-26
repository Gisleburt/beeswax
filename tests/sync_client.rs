extern crate beeswax;

mod helper;

use beeswax::resource::{Advertiser, Create, Delete, Read, Resource};
use helper::{get_sync_beeswax_client, random_string};

#[test]
fn sync_client() {
    let beeswax_client = get_sync_beeswax_client().expect("could not get beeswax client");
    let created_advertiser = Advertiser::create_builder()
        .advertiser_name(random_string("Advertiser Name"))
        .build()
        .create_sync(&beeswax_client)
        .expect("Could not create");

    let mut advertiser = Advertiser::read_builder()
        .advertiser_id(created_advertiser.advertiser_id)
        .build()
        .read_sync(&beeswax_client)
        .expect("Could not read")
        .pop()
        .expect("No advertiser found");

    advertiser.advertiser_name = random_string("Updated advertiser");

    advertiser
        .update_sync(&beeswax_client)
        .expect("Could not update");

    advertiser
        .delete_sync(&beeswax_client)
        .expect("Could not delete")
}
