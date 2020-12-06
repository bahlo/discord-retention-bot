use dotenv::dotenv;
use log::info;
use serenity::http::client::Http;
use serenity::http::GuildPagination;
use serenity::model::id::GuildId;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN")?;
    let http_client = Http::new_with_token(&discord_token);

    let guilds = http_client
        .get_guilds(&GuildPagination::After(GuildId(0)), 32)
        .await?;
    info!("guilds: {:?}", guilds);
    Ok(())
}
