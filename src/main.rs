use discord::client::DiscordNeko;

mod nekos_api;
mod discord;
mod config;

#[tokio::main]
async fn main() {
    // Create config files if they don't already exist
    config::create::initialise().expect("Failed to initialize config");

    // Load config
    let config = config::load::get().expect("Failed to load config");

    let mut neko_client = DiscordNeko::new(&config.application_id).expect("Failed to create Discord Client");
    neko_client.initialise().await.expect("Failed to initialise Discord Client");

    let mut nekos_found = 0;

    loop {
        nekos_found += 1;
        neko_client.update_assets(nekos_found).await.expect("Failed to update Discord Client");

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
