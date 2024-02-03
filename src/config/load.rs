use std::{env, error::Error, fs};

use super::model::{is_custom, Config};

pub fn get() -> Result<Config, Box<dyn Error>> {
    let config_path = if is_custom()? {
        format!("{}/catgirls", env::var("XDG_CONFIG_HOME")?)
    } else {
        format!("{}/.config/catgirls", env::var("HOME")?)
    };

    let config_file = format!("{}/config.json", config_path);

    let file = fs::File::open(&config_file)?;
    let config: Config = serde_json::from_reader(file)?;

    Ok(config)
}