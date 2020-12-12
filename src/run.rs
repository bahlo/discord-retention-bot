use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use futures::stream::{FuturesUnordered, StreamExt};
use log::{error, info};
use serenity::{
    http::{client::Http, GuildPagination},
    model::{
        channel::{ChannelType, GuildChannel, Message},
        guild::GuildInfo,
        id::GuildId,
    },
};
use std::collections::HashMap;

pub async fn run(
    client: &Http,
    channel_retention: &HashMap<String, Duration>,
    delete_pinned: bool,
) -> Result<()> {
    let guilds = get_all_guilds(&client).await?;

    let mut guild_futures = FuturesUnordered::new();
    for guild in guilds {
        guild_futures.push(process_guild(
            &client,
            guild,
            &channel_retention,
            delete_pinned,
        ));
    }

    while let Some(res) = guild_futures.next().await {
        if let Err(e) = res {
            error!("Error processing guild: {}", e);
        }
    }

    Ok(())
}

async fn get_all_guilds(client: &Http) -> Result<Vec<GuildInfo>> {
    let mut last_guild_id = Some(0u64);
    let mut guilds: Vec<GuildInfo> = vec![];
    while let Some(after) = last_guild_id {
        let mut batch = client
            .get_guilds(&GuildPagination::After(GuildId(after)), 100)
            .await?;
        guilds.append(&mut batch);
        last_guild_id = batch.last().map(|guild| *guild.id.as_u64());
    }
    Ok(guilds)
}

async fn process_guild(
    client: &Http,
    guild: GuildInfo,
    channel_retention: &HashMap<String, Duration>,
    delete_pinned: bool,
) -> Result<()> {
    info!("Processing guild {}", guild.name);
    let channels = client
        .get_channels(*guild.id.as_u64())
        .await
        .context("Could not get channels")?;
    let default_retention = channel_retention.get("*");
    for channel in channels {
        if channel.kind != ChannelType::Text {
            continue;
        }

        let max_age = match channel_retention.get(&channel.name).or(default_retention) {
            Some(max_age) => max_age,
            None => {
                info!(
                    "Skipping channel {} in guild {} as there is no configuration",
                    channel.name, guild.name
                );
                continue;
            }
        };

        match process_channel(client, &channel, *max_age, delete_pinned).await {
            Ok(num) => info!(
                "Deleted {} messages from {} in guild {}",
                num, channel.name, guild.name
            ),
            Err(e) => error!(
                "Could not process channel {} in guild {}: {:?}",
                channel.name, guild.name, e
            ),
        };
    }
    Ok(())
}

/// Gets all messages from a channel that are older than max_age and deletes
/// them. Returns the number of messages deleted.
async fn process_channel(
    client: &Http,
    channel: &GuildChannel,
    max_age: Duration,
    delete_pinned: bool,
) -> Result<u64> {
    let mut deletion_count = 0;

    let first_batch = client
        .get_messages(*channel.id.as_u64(), "?limit=100")
        .await
        .context("Could not get messages")?;
    deletion_count += delete_messages(
        client,
        channel,
        filter_messages(&first_batch, max_age, delete_pinned),
    )
    .await
    .context("Could not delete messages")?;

    let mut oldest_msg_id = first_batch.last().map(|msg| *msg.id.as_u64());
    while let Some(before_msg_id) = oldest_msg_id {
        let batch = client
            .get_messages(
                *channel.id.as_u64(),
                &format!("?limit=100&before={}", before_msg_id),
            )
            .await
            .context("Could not get messages")?;
        deletion_count += delete_messages(
            client,
            channel,
            filter_messages(&batch, max_age, delete_pinned),
        )
        .await
        .context("Could not delete messages")?;
        oldest_msg_id = batch.last().map(|msg| *msg.id.as_u64());
    }

    Ok(deletion_count)
}

fn filter_messages(messages: &[Message], max_age: Duration, delete_pinned: bool) -> Vec<u64> {
    let now = Utc::now();
    messages
        .iter()
        .filter(|msg| now.signed_duration_since(msg.timestamp) > max_age)
        .filter(|msg| delete_pinned || !msg.pinned)
        .map(|msg| *msg.id.as_u64())
        .collect()
}

/// Delete the messages with the given ids in the given channel. Returns the
/// number of messages deleted.
async fn delete_messages(
    client: &Http,
    channel: &GuildChannel,
    message_ids: Vec<u64>,
) -> Result<u64> {
    for msg_id in &message_ids {
        client
            .delete_message(*channel.id.as_u64(), *msg_id)
            .await
            .context("Could not delete message")?;
    }
    Ok(message_ids.len() as u64)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use dotenv::dotenv;
    use rand::Rng;
    use serenity::{
        client::{validate_token, Client},
        model::id::MessageId,
        utils::MessageBuilder,
    };
    use std::env;
    use tokio::runtime::Runtime;

    use super::*;

    const CHANNEL_NAME_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    #[test]
    fn test_integration_simple() {
        dotenv().ok();
        let discord_token =
            env::var("INTEGRATION_DISCORD_TOKEN").expect("INTEGRATION_DISCORD_TOKEN is unset");
        validate_token(&discord_token).expect("Token is invalid");

        Runtime::new().unwrap().block_on(async {
            let client = Client::builder(&discord_token).await.unwrap();
            let http_client = &client.cache_and_http.http;

            // Setup
            let guilds = http_client
                .get_guilds(&GuildPagination::After(GuildId(0)), 1)
                .await
                .expect("Could not get guilds");
            let guild = guilds.first().expect("Need at least one guild");

            let mut rng = rand::thread_rng();
            let channel_name: String = (0..16)
                .map(|_| {
                    let idx = rng.gen_range(0, CHANNEL_NAME_CHARSET.len());
                    CHANNEL_NAME_CHARSET[idx] as char
                })
                .collect();
            let channel = guild
                .id
                .create_channel(&http_client, |c| {
                    c.name(&channel_name).kind(ChannelType::Text)
                })
                .await
                .unwrap();

            // Add two messages and pin the latter
            channel
                .id
                .say(&http_client, MessageBuilder::new().push("foo").build())
                .await
                .unwrap();
            let bar_msg = channel
                .id
                .say(&http_client, MessageBuilder::new().push("bar").build())
                .await
                .unwrap();
            bar_msg.pin(&client.cache_and_http).await.unwrap();

            // Process channel
            let mut channel_retention = HashMap::new();
            channel_retention.insert(channel_name, Duration::seconds(0));
            run(&http_client, &channel_retention, false).await.unwrap();

            // Assert we only have one message (the pinned one)
            let messages = channel
                .id
                .messages(&http_client, |retriever| {
                    retriever.after(MessageId(0)).limit(2)
                })
                .await
                .unwrap();
            assert_eq!(1, messages.len());

            // Teardown
            http_client
                .delete_channel(*channel.id.as_u64())
                .await
                .expect("Could not delete channel");
        });
    }
}
