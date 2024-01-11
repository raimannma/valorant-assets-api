use crate::models::api_result::APIResult;
use crate::models::version::Version;

/// This function is used to get the version information from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Version.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Version>`.
pub async fn get_version(client: &reqwest::Client) -> reqwest::Result<Version> {
    client
        .get("https://valorant-api.com/v1/version")
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}
