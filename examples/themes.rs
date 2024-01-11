use static_valorant_api::models::language::Language;
use static_valorant_api::themes::{get_theme, get_themes};

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let themes = get_themes(&client, language)
        .await
        .expect("Failed to get themes");
    assert!(!themes.is_empty());

    println!(
        "Themes: {:?}",
        themes
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let theme_uuid = themes[0].uuid;
    let theme = get_theme(&client, theme_uuid, language)
        .await
        .expect("Failed to get Theme");
    assert_eq!(theme, themes[0]);
    println!("Theme: {:#?}", theme);
}
