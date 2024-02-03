use std::error::Error;

use super::{client::get_endpoint, types::{NekoImage, NekoResult}};

pub async fn get_neko() -> Result<NekoResult<NekoImage>, Box<dyn Error>> {
    get_endpoint("neko".into()).await
}