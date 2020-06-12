Beeswax
=======

An easy to use CRUD client for the [Beeswax API](https://docs.beeswax.com/docs/getting-started)

Warning
-------

This is a very early version of this crate with only a few resources so far.

ToDo:
- Add the rest of the resources
- Make runtime agnostic
- Add blocking version of the client

Usage
-----

Tell the builder what the base url you'd like to connect to is, then create an Authentication
object to send to the api to authenticate yourself.

```rust
use beeswax::{AsyncBeeswaxClient, resource::authenticate::Authenticate};

let user = std::env::var("BEESWAX_USER")?;
let password = std::env::var("BEESWAX_PASSWORD")?;
let url = "https://buzzkey.api.beeswax.com".to_string();

let beeswax_api = AsyncBeeswaxClient::builder(url)
    .auth(Authenticate::simple(user, password))
    .await?;
```

You can then create, update, read and delete [resources](beeswax::resource).

```
use beeswax::resource::Advertiser;

let create_advertiser = Advertiser::create_builder()
    .advertiser_name("Example advertiser")
    .build();

let mut created_advertiser = beeswax_api.create(&create_advertiser).await?;

created_advertiser.active = true;

let updated_advertiser = beeswax_api.update(&created_advertiser).await?;

let read_advertiser = Advertiser::read_builder()
    .advertiser_id(updated_advertiser.advertiser_id)
    .build();

let read_advertiser = beeswax_api.read(&read_advertiser).await?.pop().unwrap();

beeswax_api.delete(&read_advertiser).await?;
```
