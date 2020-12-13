#![feature(async_closure)]

use anyhow::{Context, Result};
use chrono::Duration;
use dotenv::dotenv;
use log::info;
use serenity::{client::validate_token, http::client::Http};
use std::{env, thread};

mod config;
mod run;

use run::run;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // Get, parse and validate configuration
    let discord_token = env::var("DISCORD_TOKEN").context("DISCORD_TOKEN is unset")?;
    let channel_retention = env::var("CHANNEL_RETENTION")
        .context("CHANNEL_RETENTION is unset")
        .and_then(config::parse_channel_retention)
        .context("Could not parse channel retention")?;
    let delete_pinned = env::var("DELETE_PINNED")
        .map(|val| val == "true")
        .unwrap_or(false);
    validate_token(&discord_token).context("Token is invalid")?;

    // Create client and interval
    let client = Http::new_with_token(&discord_token);
    let interval = Duration::minutes(1).to_std()?;

    // Main loop
    loop {
        run(&client, &channel_retention, delete_pinned).await?;
        info!("Sleeping for {:#?}", interval);
        thread::sleep(interval);
    }
}
