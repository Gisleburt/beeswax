//! Beeswax
//! =======
//!
//! An easy to use CRUD client for the [Beeswax API](https://docs.beeswax.com/docs/getting-started)
//!
//! Warning
//! -------
//!
//! This is a very early version of this crate with only a few resources so far.
//!
//! ToDo:
//! - Add the rest of the resources
//! - Make runtime agnostic
//! - Add blocking version of the client
//!
//! Usage
//! -----
//!
//! Tell the builder what the base url you'd like to connect to is, then create an Authentication
//! object to send to the api to authenticate yourself.
//!
//! ```no_run
//! # use std::error::Error;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! use beeswax::{BeeswaxClient, resource::Authenticate};
//!
//! let user = std::env::var("BEESWAX_USER")?;
//! let password = std::env::var("BEESWAX_PASSWORD")?;
//! let url = "https://buzzkey.api.beeswax.com".to_string();
//!
//! let beeswax_api = BeeswaxClient::builder(url)
//!     .auth(Authenticate::simple(user, password))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! You can then create, update, read and delete [resources](beeswax::resource).
//!
//! ```no_run
//! # use std::error::Error;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! # use beeswax::{BeeswaxClient, resource::Authenticate};
//! use beeswax::resource::{CreateAdvertiser, ReadAdvertiser};
//! #
//! # let user = std::env::var("BEESWAX_USER")?;
//! # let password = std::env::var("BEESWAX_PASSWORD")?;
//! # let url = "https://buzzkey.api.beeswax.com".to_string();
//! #
//! # let beeswax_api = BeeswaxClient::builder(url)
//! #     .auth(Authenticate::simple(user, password))
//! #     .await?;
//!
//! let create_advertiser = CreateAdvertiser {
//!   advertiser_name: "Example advertiser".to_string(),
//!   ..Default::default()
//! };
//!
//! let mut created_advertiser = beeswax_api.create(&create_advertiser).await?;
//!
//! created_advertiser.active = Some(true);
//!
//! let updated_advertiser = beeswax_api.update(&created_advertiser).await?;
//!
//! let read_advertiser = ReadAdvertiser {
//!   advertiser_id: Some(updated_advertiser.advertiser_id),
//!   ..Default::default()
//! };
//!
//! let read_advertiser = beeswax_api.read(&read_advertiser).await?.pop().unwrap();
//!
//! beeswax_api.delete(&read_advertiser).await?;
//! # Ok(())
//! # }
//! ```

mod client;
pub mod resource;

pub use client::{BeeswaxClient, BeeswaxClientBuilder};

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
