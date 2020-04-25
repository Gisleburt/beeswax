pub mod resource;

use crate::resource::{Authenticate, Resource, Response, SearchCriteria};
use reqwest::{Client, ClientBuilder};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct BeeswaxApiBuilder {
    base_url: String,
}

impl BeeswaxApiBuilder {
    pub async fn auth(self, auth: Authenticate) -> Result<BeeswaxApi> {
        let client = ClientBuilder::new().cookie_store(true).build()?;
        let url = format!("{}/rest/authenticate", &self.base_url);
        client.post(&url).json(&auth).send().await?;
        Ok(BeeswaxApi {
            base_url: self.base_url,
            client,
        })
    }
}

pub struct BeeswaxApi {
    base_url: String,
    client: Client,
}

impl BeeswaxApi {
    pub fn builder(base_url: String) -> BeeswaxApiBuilder {
        BeeswaxApiBuilder { base_url }
    }

    pub async fn find<R: Resource, S: SearchCriteria<R>>(&self, criteria: &S) -> Result<Vec<R>> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let request = self.client.get(&url).query(criteria).build()?;
        let response = self.client.execute(request).await?;
        let response: Response<R> = response.json().await?;
        if response.success {
            Ok(response.payload)
        } else {
            panic!("non-successful")
        }
    }
}
