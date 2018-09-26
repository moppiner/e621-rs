#[macro_use]
extern crate serde_derive;
extern crate reqwest;

pub mod error;
pub mod response;

use self::error::*;
use self::response::E621Post;

const LIST_MAX_POST_COUNT: u32 = 320;
const URL_BASE: &'static str = "https://e621.net";

pub struct E621Client {
    user_agent: String,
}

impl E621Client {
    pub fn new() -> E621Client {
        E621Client {
            user_agent: format!(
                "e621-rs/{version} (by moppiner on e621) +https://github.com/moppiner/e621-rs",
                version = option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN")
            ),
        }
    }

    pub fn list(&self, tags: String, limit: u32) -> Result<Vec<E621Post>, E621Error> {
        use reqwest::header::USER_AGENT;
        use std::cmp;

        let post_limit = cmp::min(limit, LIST_MAX_POST_COUNT);

        let client = reqwest::Client::new();
        match client
            .get(format!("{}/post/index.json", URL_BASE).as_str())
            .query(&[("limit", post_limit.to_string()), ("tags", tags)])
            .header(USER_AGENT, self.user_agent.as_str())
            .send()
        {
            Ok(mut response) => match response.json::<Vec<E621Post>>() {
                Ok(p) => Ok(p),
                Err(e) => Err(E621Error::new(e.to_string())),
            },
            Err(e) => Err(E621Error::new(e.to_string())),
        }
    }
}
