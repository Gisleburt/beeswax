use beeswax::{resource::authenticate::Authenticate, AsyncBeeswaxClient};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::error::Error;

pub async fn get_beeswax_client() -> Result<AsyncBeeswaxClient, Box<dyn Error>> {
    let user = std::env::var("BEESWAX_USER")?;
    let password = std::env::var("BEESWAX_PASSWORD")?;
    let url = std::env::var("BEESWAX_URL")?;

    Ok(AsyncBeeswaxClient::builder(url)
        .auth(Authenticate::simple(user, password))
        .await?)
}

pub fn random_string(prefix: &str) -> String {
    let random_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    format!("{} {}", prefix, random_string)
}
