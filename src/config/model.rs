use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub application_id: String,
}

// IDK where to put this lmao
pub fn is_custom() -> Result<bool, VarError> {
    if let Ok(_) = env::var("XDG_CONFIG_HOME") {
        return Ok(true);
    }

    return Ok(false);
}