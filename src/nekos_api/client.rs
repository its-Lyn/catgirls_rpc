use std::error::Error;

use serde::de::DeserializeOwned;

pub async fn get_endpoint<T>(endpoint: String) -> Result<T, Box<dyn Error>>
where T: DeserializeOwned, {
    let url = format!("https://nekos.best/api/v2/{}", endpoint);
    let json = reqwest::get(url)
        .await?
        .json()
        .await?;

    Ok(
        serde_json::from_value(json)?
    )
}