use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct NekoResult<T> {
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize)]
pub struct NekoImage {
    pub artist_href: String,
    pub artist_name: String,
    
    pub source_url: String,
    pub url: String,
}