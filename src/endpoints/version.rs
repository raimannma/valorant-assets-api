use crate::models::api_result::APIResult;
use crate::models::version::Version;

pub async fn get_version(client: &reqwest::Client) -> reqwest::Result<Version> {
    client
        .get("https://valorant-api.com/v1/version")
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}
