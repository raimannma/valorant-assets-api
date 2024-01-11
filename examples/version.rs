use static_valorant_api::version::get_version;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let version = get_version(&client).await.expect("Failed to get version");
    println!("Version: {:#?}", version);
}
