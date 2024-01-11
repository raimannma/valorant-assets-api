use crate::models::api_result::APIResult;
use crate::models::language::Language;
use crate::models::weapon::{Weapon, WeaponSkin, WeaponSkinChroma, WeaponSkinLevels};
use url::Url;
use uuid::Uuid;

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
