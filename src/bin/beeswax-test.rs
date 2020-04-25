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
    let advertiser = beeswax_api.find(&criteria).await.unwrap();
    dbg!(advertiser);
}

fn get_env_var(name: &str) -> String {
    std::env::var(name).expect(format!("{} not defined", name).as_str())
}
