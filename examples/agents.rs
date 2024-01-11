use valorant_assets_api::agents::{get_agent, get_agents};
use valorant_assets_api::models::language::Language;

#[tokio::main]
async fn main() {
    let language = Some(Language::DeDe);

    let client = reqwest::Client::new();
    let agents = get_agents(&client, language, None)
        .await
        .expect("Failed to get agents");
    assert!(!agents.is_empty());

    println!(
        "Agents: {:?}",
        agents
            .iter()
            .map(|x| x.display_name.clone())
            .collect::<Vec<String>>()
    );

    let agent_uuid = agents[0].uuid;
    let agent = get_agent(&client, agent_uuid, language)
        .await
        .expect("Failed to get agent");
    assert_eq!(agent, agents[0]);
    println!("Agent: {:#?}", agent);
}
