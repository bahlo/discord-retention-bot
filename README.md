# discord-retention-bot ![ci](https://github.com/bahlo/discord-retention-bot/workflows/ci/badge.svg)
A discord bot that allows you to set a retention for different channels (similar to Slack)

## Install
1. Create a Discord Application in the 
   [Discord Developer Portal](https://discord.com/developers/applications)
2. Copy the client id from the "General information" tab
3. Go to <https://discord.com/oauth2/authorize?client_id=$CLIENT_ID&scope=bot&permissions=74752>
   and add your Bot to your discord server.

### What does 74752 mean?
74752 is the bot permission bitmask for the following permissions:
* View Channels
* Manage Messages
* Read Message History

## Configuration

Configuration is happening via environment variables (optionally via `.env`).

* `RUST_LOG` defines the log level (I recommend setting this to `INFO`)
* `DISCORD_TOKEN` is the token of your Discord bot
* `CHANNEL_RETENTION` is a list of channel names and the duration to keep, separated by
  a comma. For example: `general:2w,random:4d`. 

## Troubleshooting
### Why is it taking so long?
Discord might be rate-limiting you. This applications uses the single message
delete endpoint because [Bulk Delete Messages](https://discord.com/developers/docs/resources/channel#bulk-delete-messages) doesn't support messages older than 2 weeks. 
It might take a while the first time, but it will get faster.

## It's not deleting the messages of a channel
Make sure the bot has access to that channel in the Discord application.