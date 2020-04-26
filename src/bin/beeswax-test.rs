use beeswax_rs::resource::*;
use beeswax_rs::BeeswaxApi;

#[tokio::main]
async fn main() {
    let user = get_env_var("BEESWAX_USER");
    let password = get_env_var("BEESWAX_PASSWORD");

    let beeswax_api = BeeswaxApi::builder("https://triptease.api.beeswax.com".to_string())
        .auth(Authenticate::simple(user, password))
        .await
        .unwrap();
    let criteria = AdvertiserSearchCriteria {
        advertiser_id: Some(496),
        ..Default::default()
    };
    let mut advertiser = beeswax_api.find(&criteria).await.unwrap();
    // dbg!(advertiser);
    let mut creative_criteria: CreativeSearchCriteria = advertiser.pop().unwrap().into();
    creative_criteria.creative_name = Some("2019-20200306-medium_large_rectangle 300x250".to_string());
    let creatives = beeswax_api.find(&creative_criteria).await.unwrap();
    dbg!(creatives);
}

fn get_env_var(name: &str) -> String {
    std::env::var(name).expect(format!("{} not defined", name).as_str())
}
