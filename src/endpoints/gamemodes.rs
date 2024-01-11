use crate::models::api_result::APIResult;
use crate::models::gamemode::{Gamemode, GamemodeEquippable};
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

pub async fn get_gamemodes(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Gamemode>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gamemodes").unwrap();
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

pub async fn get_gamemode(
    client: &reqwest::Client,
    gamemode: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Gamemode> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/gamemodes/{}",
        gamemode
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

pub async fn get_gamemode_equippables(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<GamemodeEquippable>> {
    let mut url = Url::parse("https://valorant-api.com/v1/gamemodes/equippables").unwrap();
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

pub async fn get_gamemode_equippable(
    client: &reqwest::Client,
    equippable: Uuid,
    language: Option<Language>,
) -> reqwest::Result<GamemodeEquippable> {
    let mut url = Url::parse(&format!(
        "https://valorant-api.com/v1/gamemodes/equippables/{}",
        equippable
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
