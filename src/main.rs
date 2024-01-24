use discord::client::DiscordNeko;

mod nekos_api;
mod discord;
mod config;

#[tokio::main]
async fn main() {
    // TODO: Add a config option to fetch ID
    let mut neko_client = DiscordNeko::new("-").expect("Failed to create Discord Client");
    neko_client.initialise().await.expect("Failed to initialise Discord Client");

    let mut nekos_found = 0;

    loop {
        nekos_found += 1;
        neko_client.update_assets(nekos_found).await.expect("Failed to update Discord Client");

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
