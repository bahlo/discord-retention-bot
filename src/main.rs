use anyhow::{Context, Result};
use chrono::{prelude::*, Duration};
use dotenv::dotenv;
use log::{error, info};
use serenity::{
    http::{client::Http, GuildPagination},
    model::{
        channel::{GuildChannel, Message},
        guild::GuildInfo,
        id::GuildId,
    },
};
use std::{collections::HashMap, env, thread};

mod config;
mod errors;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let discord_token = env::var("DISCORD_TOKEN").context("DISCORD_TOKEN is unset")?;
    let channel_retention_env =
        env::var("CHANNEL_RETENTION").context("CHANNEL_RETENTION is unset")?;
    let channel_retention = config::parse_channel_retention(channel_retention_env)
        .context("Could not parse channel retention")?;
    let client = Http::new_with_token(&discord_token);
    let interval = Duration::minutes(1).to_std()?;

    loop {
        let guilds = get_all_guilds(&client).await?;

        for guild in guilds {
            info!("Processing guild {}", guild.name);
            if let Err(e) = process_guild(&client, &guild, &channel_retention).await {
                error!("Could not process guild {}: {:?}", guild.name, e)
            }
        }

        info!("Sleeping for {:#?}", interval);
        thread::sleep(interval);
    }
}

async fn get_all_guilds(client: &Http) -> Result<Vec<GuildInfo>> {
    let mut last_guild_id = Some(0u64);
    let mut guilds: Vec<GuildInfo> = vec![];
    while let Some(after) = last_guild_id {
        let mut batch = client
            .get_guilds(&GuildPagination::After(GuildId(after)), 100)
            .await?;
        guilds.append(&mut batch);
        last_guild_id = batch.last().and_then(|guild| Some(*guild.id.as_u64()));
    }
    Ok(guilds)
}

async fn process_guild(
    client: &Http,
    guild: &GuildInfo,
    channel_retention: &HashMap<String, Duration>,
) -> Result<()> {
    let channels = client
        .get_channels(*guild.id.as_u64())
        .await
        .context("Could not get channels")?;
    for channel in channels {
        let max_age = match channel_retention.get(&channel.name) {
            Some(max_age) => max_age,
            None => {
                info!(
                    "Skipping channel {} as there is no configuration",
                    channel.name
                );
                continue;
            }
        };

        if let Err(e) = process_channel(client, &channel, *max_age).await {
            error!(
                "Could not process channel {} in guild {}: {:?}",
                channel.name, guild.name, e
            )
        }
    }
    Ok(())
}

async fn process_channel(client: &Http, channel: &GuildChannel, max_age: Duration) -> Result<()> {
    let first_batch = client
        .get_messages(*channel.id.as_u64(), "?limit=100")
        .await
        .context("Could not get messages")?;
    delete_messages(client, channel, filter_messages(&first_batch, max_age))
        .await
        .context("Could not delete messages")?;

    let mut oldest_msg = first_batch.last();
    #[allow(unused_assignments)] // TODO: Get rid of this and the batch var
    let mut batch: Vec<Message> = vec![];
    while let Some(before_msg) = oldest_msg {
        batch = client
            .get_messages(
                *channel.id.as_u64(),
                &format!("?limit=100&before={}", before_msg.id.as_u64()),
            )
            .await
            .context("Could not get messages")?;
        delete_messages(client, channel, filter_messages(&batch, max_age))
            .await
            .context("Could not delete messages")?;
        oldest_msg = batch.last();
    }

    Ok(())
}

fn filter_messages(messages: &Vec<Message>, max_age: Duration) -> Vec<u64> {
    let now = Utc::now();
    messages
        .into_iter()
        .filter(|msg| return now.signed_duration_since(msg.timestamp) > max_age)
        .map(|msg| *msg.id.as_u64())
        .collect()
}

async fn delete_messages(
    client: &Http,
    channel: &GuildChannel,
    message_ids: Vec<u64>,
) -> Result<()> {
    for msg_id in message_ids {
        client
            .delete_message(*channel.id.as_u64(), msg_id)
            .await
            .context("Could not delete message")?;
    }
    Ok(())
}
