use static_valorant_api::models::language::Language;
use static_valorant_api::weapons::{
    get_weapon, get_weapon_skinchroma, get_weapon_skinchromas, get_weapon_skinlevel,
    get_weapon_skinlevels, get_weapons,
};

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let weapons = get_weapons(&client, language)
        .await
        .expect("Failed to get weapons");
    assert!(!weapons.is_empty());

    println!(
        "Weapons: {:?}",
        weapons
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let weapon_uuid = weapons[0].uuid;
    let weapon = get_weapon(&client, weapon_uuid, language)
        .await
        .expect("Failed to get Weapon");
    assert_eq!(weapon, weapons[0]);
    println!("Weapon: {:#?}", weapon);

    let weapon_skinchromas = get_weapon_skinchromas(&client, language)
        .await
        .expect("Failed to get weapon skinchromas");
    assert!(!weapon_skinchromas.is_empty());

    println!(
        "Weapons Skinchromas: {:?}",
        weapon_skinchromas
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let weapon_skinchroma_uuid = weapon_skinchromas[0].uuid;
    let weapon_skinchroma = get_weapon_skinchroma(&client, weapon_skinchroma_uuid, language)
        .await
        .expect("Failed to get weapon skinchroma");
    assert_eq!(weapon_skinchroma, weapon_skinchromas[0]);
    println!("Weapon Skinchroma: {:#?}", weapon_skinchroma);

    let weapon_skinlevels = get_weapon_skinlevels(&client, language)
        .await
        .expect("Failed to get weapon skinlevels");
    assert!(!weapon_skinlevels.is_empty());

    println!(
        "Weapons Skinlevels: {:?}",
        weapon_skinlevels
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let weapon_skinlevel_uuid = weapon_skinlevels[0].uuid;
    let weapon_skinlevel = get_weapon_skinlevel(&client, weapon_skinlevel_uuid, language)
        .await
        .expect("Failed to get weapon skinlevel");
    assert_eq!(weapon_skinlevel, weapon_skinlevels[0]);
    println!("Weapon Skinlevel: {:#?}", weapon_skinlevel);
}
