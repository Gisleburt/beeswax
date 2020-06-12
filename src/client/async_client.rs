use isahc::{HttpClient, ResponseExt};
use rand::Rng;
use serde_json;
use serde_urlencoded::to_string as to_url;

use crate::resource::{
    authenticate::Authenticate, AnyResource, Create, Delete, Read, Resource, ResponseId,
    ResponseResource,
};
use crate::Result;
use isahc::prelude::Request;

/// Creates the BeeswaxApi client. This type is instantiated from the BeeswaxApi struct.
pub struct AsyncBeeswaxClientBuilder {
    base_url: String,
}

impl AsyncBeeswaxClientBuilder {
    pub async fn auth(self, auth: Authenticate) -> Result<AsyncBeeswaxClient> {
        let client = HttpClient::builder().cookies().build()?;
        let url = format!("{}/rest/authenticate", &self.base_url);
        let body = serde_json::to_vec(&auth)?;
        client.post_async(&url, body).await?;
        Ok(AsyncBeeswaxClient {
            base_url: self.base_url,
            client,
        })
    }
}

/// Provides an interface to the Beeswax Api
pub struct AsyncBeeswaxClient {
    base_url: String,
    client: HttpClient,
}

impl AsyncBeeswaxClient {
    /// Creates the API builder
    pub fn builder(base_url: String) -> AsyncBeeswaxClientBuilder {
        AsyncBeeswaxClientBuilder { base_url }
    }

    /// Find resources based on a search criteria
    pub async fn read<R: Resource, F: Read<R>>(&self, criteria: &F) -> Result<Vec<R>> {
        let url = format!("{}/rest/{}?{}", &self.base_url, R::NAME, to_url(criteria)?);
        let mut response = self.client.get_async(&url).await?;
        let response: ResponseResource<R> = response.json()?;
        if response.success {
            Ok(response.payload)
        } else {
            panic!("non-successful")
        }
    }

    /// Create a given resource
    pub async fn create<R: Resource, C: Create<R>>(&self, create: &C) -> Result<R> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let body = serde_json::to_vec(&create)?;
        let mut response = self.client.post_async(&url, body.clone()).await?;
        if !response.status().is_success() {
            dbg!(body);
            dbg!(response.text());
            panic!("nope");
        }
        let response: ResponseId = response.json()?;
        if response.success {
            Ok(create.clone().into_resource(response.payload.id))
        } else {
            panic!("non-successful")
        }
    }

    /// Update a given resource
    pub async fn update<'a, R: Resource>(&self, resource: &'a R) -> Result<&'a R> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let body = serde_json::to_vec(&resource)?;
        let mut response = self.client.put_async(&url, body).await?;

        if !response.status().is_success() {
            dbg!(response.text());
            panic!("nope");
        }

        Ok(resource)
    }

    /// Delete a given resource
    pub async fn delete<R: Resource, D: Delete<R>>(&self, delete: &D) -> Result<()> {
        let url = format!("{}/rest/{}", &self.base_url, R::NAME);
        let body = serde_json::to_vec(&delete)?;
        let request = Request::delete(url)
            .header("Content-Type", "application/json")
            .body(body)?;
        let mut response = self.client.send_async(request).await?;

        if !response.status().is_success() {
            dbg!(response.text());
            panic!("nope");
        }

        Ok(())
    }
}

pub struct AsyncInMemoryClient {
    store: Vec<AnyResource>,
}

impl AsyncInMemoryClient {
    pub fn new() -> AsyncInMemoryClient {
        AsyncInMemoryClient { store: Vec::new() }
    }

    pub async fn read<R: Resource, F: Read<R>>(&self, criteria: &F) -> Result<Vec<R>> {
        let vec = self
            .store
            .iter() // Inefficient
            .filter_map(|r| R::from_any_resource(r))
            .filter(|r| &criteria == r)
            .map(|r| r.clone())
            .collect();
        Ok(vec)
    }

    pub async fn create<R: Resource, C: Create<R>>(&mut self, create: &C) -> Result<R> {
        let mut rng = rand::thread_rng();
        let resource = create.clone().into_resource(rng.gen_range(1, 100000));
        self.store.push(resource.clone().into());
        Ok(resource)
    }

    pub async fn update<'a, R: Resource>(&self, resource: &'a R) -> Result<&'a R> {
        unimplemented!()
    }

    pub async fn delete<R: Resource, D: Delete<R>>(&self, delete: &D) -> Result<()> {
        Ok(())
    }
}
