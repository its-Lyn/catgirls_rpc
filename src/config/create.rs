use std::{env, error::Error, fs, path::Path};

use serde_json::to_writer_pretty;

use super::model::{is_custom, Config};

pub fn initialise() -> Result<(), Box<dyn Error>> {
    let base = if is_custom()? {
        env::var("XDG_CONFIG_HOME")?
    } else {
        format!("{}/.config", env::var("HOME")?)
    };

    let config_path = format!("{}/catgirls", base);
    if !Path::new(&config_path).exists() {
        fs::create_dir(&config_path)?;
    }

    let config_file = format!("{}/config.json", config_path);
    if !Path::new(&config_file).exists() {
        let default_config = Config {
            application_id: String::new(),
        };

        let file = fs::File::create(&config_file)?;
        to_writer_pretty(file, &default_config)?;
    }
    
    Ok(())
}