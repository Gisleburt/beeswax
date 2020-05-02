use beeswax::resource::*;
use beeswax::BeeswaxClient;

#[tokio::main]
async fn main() {
    let user = get_env_var("BEESWAX_USER");
    let password = get_env_var("BEESWAX_PASSWORD");

    let beeswax_api = BeeswaxClient::builder("https://triptease.api.beeswax.com".to_string())
        .auth(Authenticate::simple(user, password))
        .await
        .unwrap();
    let criteria = ReadAdvertiser {
        advertiser_id: Some(496),
        ..Default::default()
    };
    let mut advertiser = beeswax_api.read(&criteria).await.unwrap();
    // dbg!(advertiser);
    let mut creative_criteria: ReadCreative = advertiser.pop().unwrap().into();
    creative_criteria.creative_name =
        Some("2019-20200306-medium_large_rectangle 300x250".to_string());
    let creatives = beeswax_api.read(&creative_criteria).await.unwrap();
    dbg!(creatives);
}

fn get_env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{} not defined", name))
}
