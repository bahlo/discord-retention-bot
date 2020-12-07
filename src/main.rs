use dotenv::dotenv;
use log::{error, info};
use serenity::http::client::Http;
use serenity::http::GuildPagination;
use serenity::model::channel::GuildChannel;
use serenity::model::guild::GuildInfo;
use serenity::model::id::GuildId;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN")?;
    let client = Http::new_with_token(&discord_token);

    let guilds = client
        .get_guilds(&GuildPagination::After(GuildId(0)), 1)
        .await?;

    for guild in guilds {
        info!("Processing guild {}", guild.name);
        if let Err(e) = process_guild(&client, &guild).await {
            error!("Could not process guild {}: {:?}", guild.name, e)
        }
    }

    Ok(())
}

async fn process_guild(client: &Http, guild: &GuildInfo) -> Result<(), Box<dyn std::error::Error>> {
    let channels = client.get_channels(*guild.id.as_u64()).await?;
    for channel in channels {
        if let Err(e) = process_channel(client, &channel).await {
            error!(
                "Could not process channel {} in guild {}: {:?}",
                channel.name, guild.name, e
            )
        }
    }
    Ok(())
}

async fn process_channel(
    client: &Http,
    channel: &GuildChannel,
) -> Result<(), Box<dyn std::error::Error>> {
    let messages = client
        .get_messages(*channel.id.as_u64(), "?limit=100")
        .await?;
    info!("got {} messages in {}", messages.len(), channel.name);
    Ok(())
}
