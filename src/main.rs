use chrono::prelude::*;
use chrono::Duration;
use dotenv::dotenv;
use log::{error, info};
use serenity::http::client::Http;
use serenity::http::GuildPagination;
use serenity::model::channel::{GuildChannel, Message};
use serenity::model::guild::GuildInfo;
use serenity::model::id::GuildId;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN")?;
    let client = Http::new_with_token(&discord_token);

    // TODO: Make configurable
    let max_age = Duration::weeks(2);

    let guilds = client
        .get_guilds(&GuildPagination::After(GuildId(0)), 1)
        .await?;

    for guild in guilds {
        info!("Processing guild {}", guild.name);
        if let Err(e) = process_guild(&client, &guild, max_age).await {
            error!("Could not process guild {}: {:?}", guild.name, e)
        }
    }

    Ok(())
}

async fn process_guild(
    client: &Http,
    guild: &GuildInfo,
    max_age: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let channels = client.get_channels(*guild.id.as_u64()).await?;
    for channel in channels {
        if let Err(e) = process_channel(client, &channel, max_age).await {
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
    max_age: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let first_batch = client
        .get_messages(*channel.id.as_u64(), "?limit=100")
        .await?;

    let message_ids_to_delete: Vec<u64> = filter_messages(first_batch, max_age);

    // TODO: Implement pagination loop to get all messages, append to message_ids_to_delet

    // TODO: Delete messages with ids
    info!("Message ids to be deleted: {:?}", message_ids_to_delete);

    Ok(())
}

fn filter_messages(messages: Vec<Message>, max_age: Duration) -> Vec<u64> {
    let now = Utc::now();
    messages
        .into_iter()
        .filter(|msg| return now.signed_duration_since(msg.timestamp) > max_age)
        .map(|msg| *msg.id.as_u64())
        .collect()
}
