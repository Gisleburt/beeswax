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
//! beeswax_api.delete(read_advertiser).await?;
//! # Ok(())
//! # }
//! ```

pub mod resource;

use crate::resource::{Authenticate, Create, Delete, Read, Resource, ResponseId, ResponseResource};
use reqwest::{Client, ClientBuilder};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Creates the BeeswaxApi client. This type is instantiated from the BeeswaxApi struct.
pub struct BeeswaxApiBuilder {
    base_url: String,
}

impl BeeswaxApiBuilder {
    pub async fn auth(self, auth: Authenticate) -> Result<BeeswaxClient> {
        let client = ClientBuilder::new().cookie_store(true).build()?;
        let url = format!("{}/rest/authenticate", &self.base_url);
        client.post(&url).json(&auth).send().await?;
        Ok(BeeswaxClient {
            base_url: self.base_url,
            client,
        })
    }
}

/// Provides an interface to the Beeswax Api
pub struct BeeswaxClient {
    base_url: String,
    client: Client,
}

impl BeeswaxClient {
    /// Creates the API builder
    pub fn builder(base_url: String) -> BeeswaxApiBuilder {
        BeeswaxApiBuilder { base_url }
    }

    /// Find resources based on a search criteria
    pub async fn read<R: Resource, F: Read<R>>(&self, criteria: &F) -> Result<Vec<R>> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let request = self.client.get(&url).query(criteria).build()?;
        let response = self.client.execute(request).await?;
        let response: ResponseResource<R> = response.json().await?;
        if response.success {
            Ok(response.payload)
        } else {
            panic!("non-successful")
        }
    }

    /// Create a given resource
    pub async fn create<R: Resource, C: Create<R>>(&self, create: &C) -> Result<R> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let request = self.client.post(&url).json(&create).build()?;
        let response = self.client.execute(request).await?;
        let response: ResponseId = response.json().await?;
        if response.success {
            Ok(create.clone().into_resource(response.payload.id))
        } else {
            panic!("non-successful")
        }
    }

    /// Update a given resource
    pub async fn update<'a, R: Resource>(&self, resource: &'a R) -> Result<&'a R> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let request = self.client.put(&url).json(&resource).build()?;
        let response = self.client.execute(request).await?;
        let response: ResponseId = response.json().await?;
        if response.success {
            Ok(resource)
        } else {
            panic!("non-successful")
        }
    }

    /// Delete a given resource
    pub async fn delete<R: Resource, D: Delete<R>>(&self, delete: D) -> Result<()> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let request = self.client.delete(&url).json(&delete).build()?;
        let response = self.client.execute(request).await?;
        let response: ResponseId = response.json().await?;
        if response.success {
            Ok(())
        } else {
            panic!("non-successful")
        }
    }
}
