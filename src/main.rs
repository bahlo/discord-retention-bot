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
        .get_guilds(&GuildPagination::After(GuildId(0)), 1)
        .await?;
    let first_guild = guilds.get(0).expect("No guild provided"); // TODO: Support multiple guilds
    info!("guild: {:?}", first_guild);

    let channels = http_client.get_channels(*first_guild.id.as_u64()).await?;
    info!("channels: {:?}", channels);

    Ok(())
}
