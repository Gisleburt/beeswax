extern crate beeswax;

use beeswax::resource::{common::ViewName, view::ReadView};

mod helper;

#[tokio::test]
async fn test_view() {
    let beeswax_client = helper::get_async_beeswax_client().await.unwrap();

    let read_view = ReadView {
        view_name: ViewName::Continents,
    };

    let _response = beeswax_client.read(&read_view).await.unwrap();
    // dbg!(_response);

    let read_view = ReadView {
        view_name: ViewName::Currency,
    };

    let _response = beeswax_client.read(&read_view).await.unwrap();
    // dbg!(_response);
}
