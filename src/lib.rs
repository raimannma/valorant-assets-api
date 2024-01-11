//! # Rust bindings for the [Valorant Assets API](https://valorant-api.com/)
//!
//! This crate provides Rust bindings for the [Valorant Assets API](https://valorant-api.com/).
//!
//! ## Installation
//!
//! Install this crate with `cargo add valorant-assets-api` or add it to your `Cargo.toml`.
//!
//! ## Usage
//!
//! ```rust
//! use valorant_assets_api::agents::{get_agent, get_agents};
//! use valorant_assets_api::models::language::Language;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a reqwest::Client, which is used to send HTTP requests.
//!     let client = reqwest::Client::new();
//!
//!     // Get a list of agents from the Valorant API. (language is optional)
//!     let agents = get_agents(&client, Some(Language::DeDe), None)
//!         .await
//!         .expect("Failed to get agents");
//!
//!     println!(
//!         "Agents: {:?}",
//!         agents
//!             .iter()
//!             .map(|x| x.display_name.clone())
//!             .collect::<Vec<_>>()
//!     );
//!
//!     println!(
//!         "Single Agent: {:#?}",
//!         agents.first()
//!     );
//! }
//! ```

pub mod endpoints;
pub mod models;

pub use endpoints::*;
