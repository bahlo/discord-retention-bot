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
use std::ops::Sub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN")?;
    let client = Http::new_with_token(&discord_token);

    // TODO: Make timerange configurable
    let two_weeks_ago = Utc::now().sub(Duration::weeks(2));

    let guilds = client
        .get_guilds(&GuildPagination::After(GuildId(0)), 1)
        .await?;

    for guild in guilds {
        info!("Processing guild {}", guild.name);
        if let Err(e) = process_guild(&client, &guild, two_weeks_ago).await {
            error!("Could not process guild {}: {:?}", guild.name, e)
        }
    }

    Ok(())
}

async fn process_guild(
    client: &Http,
    guild: &GuildInfo,
    delete_before: DateTime<Utc>,
) -> Result<(), Box<dyn std::error::Error>> {
    let channels = client.get_channels(*guild.id.as_u64()).await?;
    for channel in channels {
        if let Err(e) = process_channel(client, &channel, delete_before).await {
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
    delete_before: DateTime<Utc>,
) -> Result<(), Box<dyn std::error::Error>> {
    let first_batch = client
        .get_messages(*channel.id.as_u64(), "?limit=100")
        .await?;

    let message_ids_to_delete: Vec<u64> = filter_messages(first_batch, delete_before)?;

    // TODO: Implement pagination loop to get all messages, append to message_ids_to_delet

    // TODO: Delete messages with ids
    info!("Message ids to be deleted: {:?}", message_ids_to_delete);

    Ok(())
}

fn filter_messages(
    _messages: Vec<Message>,
    _before: DateTime<Utc>,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    // TODO: To filtering
    Ok(vec![])
}
