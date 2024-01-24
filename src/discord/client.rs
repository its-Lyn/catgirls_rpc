use std::error::Error;

use chrono::{Duration, Utc};
use discord_rich_presence::{activity::{Activity, Assets, Timestamps}, DiscordIpc, DiscordIpcClient};

use crate::nekos_api::nekos;

const WAIT_TIME: i32 = 15;

pub struct DiscordNeko {
    default_details: String,
    default_state: String,

    client: DiscordIpcClient
}

impl DiscordNeko {
    pub fn new(id: &str) -> Result<DiscordNeko, Box<dyn Error>> {
        Ok(DiscordNeko {
            default_details: "Looking at Nekos!".into(),
            default_state: "Nya~ >w<".into(),

            client: DiscordIpcClient::new(id)?
        })
    }

    pub async fn initialise(&mut self) -> Result<(), Box<dyn Error>> {
        if let Err(e) = self.client.connect() {
            eprintln!("Failed to connect to Discord: {}", e);
            loop {
                eprintln!("Trying again in {} seconds...", WAIT_TIME);
                tokio::time::sleep(std::time::Duration::from_secs(WAIT_TIME as u64)).await;

                if let Ok(()) = self.client.connect() {
                    eprintln!("Connection established!");   
                    break;
                } 
            }
        }

        Ok(())
    }

    pub async fn update_assets(&mut self, neko_count: i32) -> Result<(), Box<dyn Error>> {
        // Get a shortened version of the API call.
        let sources = DiscordNeko::get_sources().await;

        // Get unix time and future
        let current = Utc::now();
        let change = current + Duration::seconds(60);

        self.client.set_activity(
            Activity::new()
                .details(&format!("{} ({})", &self.default_details, neko_count))
                .state(&self.default_state)
                .assets(
                    Assets::new()
                        .large_image(&sources.0)
                        .large_text(&sources.1)
                )
                .timestamps(
                    Timestamps::new()
                        .start(current.timestamp())
                        .end(change.timestamp())
                )
        )?;
        
        Ok(())
    }

    async fn get_sources() -> (String, String) {
        let neko = nekos::get_neko().await.expect("Failed to make API call");
        let result = &neko.results[0];

        (result.url.to_string(), result.artist_name.to_string())
    }
}