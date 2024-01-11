use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::weapon::{Weapon, WeaponSkin, WeaponSkinChroma, WeaponSkinLevels};
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of weapons from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Weapons.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Weapon>>`.
pub async fn get_weapons(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Weapon>> {
    let mut url = Url::parse("https://valorant-api.com/v1/weapons").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a specific weapon from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `weapon` - A Uuid that represents the unique identifier of the weapon.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Weapon.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Weapon>`.
pub async fn get_weapon(
    client: &reqwest::Client,
    weapon: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Weapon> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/weapons/{}", weapon)).unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a list of weapon skins from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of WeaponSkins.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<WeaponSkin>>`.
pub async fn get_weapon_skins(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<WeaponSkin>> {
    let mut url = Url::parse("https://valorant-api.com/v1/weapons/skins").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a specific weapon skin from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `skin` - A Uuid that represents the unique identifier of the weapon skin.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a WeaponSkin.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<WeaponSkin>`.
pub async fn get_weapon_skin(
    client: &reqwest::Client,
    skin: Uuid,
    language: Option<Language>,
) -> reqwest::Result<WeaponSkin> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/weapons/skins/{}",
        skin
    ))
    .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a list of weapon skin chromas from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of WeaponSkinChromas.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<WeaponSkinChroma>>`.
pub async fn get_weapon_skinchromas(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<WeaponSkinChroma>> {
    let mut url = Url::parse("https://valorant-api.com/v1/weapons/skinchromas").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a specific weapon skin chroma from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `skinchroma` - A Uuid that represents the unique identifier of the weapon skin chroma.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a WeaponSkinChroma.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<WeaponSkinChroma>`.
pub async fn get_weapon_skinchroma(
    client: &reqwest::Client,
    skinchroma: Uuid,
    language: Option<Language>,
) -> reqwest::Result<WeaponSkinChroma> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/weapons/skinchromas/{}",
        skinchroma
    ))
    .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a list of weapon skin levels from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of WeaponSkinLevels.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<WeaponSkinLevels>>`.
pub async fn get_weapon_skinlevels(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<WeaponSkinLevels>> {
    let mut url = Url::parse("https://valorant-api.com/v1/weapons/skinlevels").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a specific weapon skin level from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `skinlevel` - A Uuid that represents the unique identifier of the weapon skin level.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a WeaponSkinLevels.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<WeaponSkinLevels>`.
pub async fn get_weapon_skinlevel(
    client: &reqwest::Client,
    skinlevel: Uuid,
    language: Option<Language>,
) -> reqwest::Result<WeaponSkinLevels> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/weapons/skinlevels/{}",
        skinlevel
    ))
    .unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}
