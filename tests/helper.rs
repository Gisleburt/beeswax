use beeswax::{resource::authenticate::Authenticate, AsyncBeeswaxClient};
use std::error::Error;

pub async fn get_beeswax_client() -> Result<AsyncBeeswaxClient, Box<dyn Error>> {
    let user = std::env::var("BEESWAX_USER")?;
    let password = std::env::var("BEESWAX_PASSWORD")?;
    let url = std::env::var("BEESWAX_URL")?;

    Ok(AsyncBeeswaxClient::builder(url)
        .auth(Authenticate::simple(user, password))
        .await?)
}
