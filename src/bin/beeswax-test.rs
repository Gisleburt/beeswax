use beeswax_rs::resource::Authenticate;
use beeswax_rs::BeeswaxApi;

#[tokio::main]
async fn main() {
    let user = get_env_var("BEESWAX_USER");
    let password = get_env_var("BEESWAX_PASSWORD");

    let _beeswax_api = BeeswaxApi::builder("https://triptease.api.beeswax.com".to_string())
        .auth(Authenticate::simple(user, password))
        .await
        .unwrap();
    println!("Whoop");
}

fn get_env_var(name: &str) -> String {
    std::env::var(name).expect(format!("{} not defined", name).as_str())
}
