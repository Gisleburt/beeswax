use crate::resource::{
    authenticate::Authenticate, Create, Delete, Read, Resource, ResponseId, ResponseResource,
};
use reqwest::{Client, ClientBuilder};

use crate::Result;

/// Creates the BeeswaxApi client. This type is instantiated from the BeeswaxApi struct.
pub struct AsyncBeeswaxClientBuilder {
    base_url: String,
}

impl AsyncBeeswaxClientBuilder {
    pub async fn auth(self, auth: Authenticate) -> Result<AsyncBeeswaxClient> {
        let client = ClientBuilder::new().cookie_store(true).build()?;
        let url = format!("{}/rest/authenticate", &self.base_url);
        client.post(&url).json(&auth).send().await?;
        Ok(AsyncBeeswaxClient {
            base_url: self.base_url,
            client,
        })
    }
}

/// Provides an interface to the Beeswax Api
pub struct AsyncBeeswaxClient {
    base_url: String,
    client: Client,
}

impl AsyncBeeswaxClient {
    /// Creates the API builder
    pub fn builder(base_url: String) -> AsyncBeeswaxClientBuilder {
        AsyncBeeswaxClientBuilder { base_url }
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
    pub async fn delete<R: Resource, D: Delete<R>>(&self, delete: &D) -> Result<()> {
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
