extern crate dotenv;
extern crate env_logger;
extern crate log;

use dotenv::dotenv;
use log::debug;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN")?;

    debug!("discord_token: {}", discord_token);
    Ok(())
}
