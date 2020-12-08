# discord-retention-bot 
![ci](https://github.com/bahlo/discord-retention-bot/workflows/ci/badge.svg)
![release](https://github.com/bahlo/discord-retention-bot/workflows/release/badge.svg)

A discord bot that allows you to set a retention for different channels (similar to Slack)

## Install
### Preparation
Before running your bot you need to create it on Discord:

1. Create a Discord Application in the 
   [Discord Developer Portal](https://discord.com/developers/applications)
2. Copy the client id from the "General information" tab
3. Go to <https://discord.com/oauth2/authorize?client_id=$CLIENT_ID&scope=bot&permissions=74752>
   and add your Bot to your discord server.

#### Wait what does 74752 mean?
74752 is the bitmask for the following permissions:
* View Channels
* Manage Messages
* Read Message History

### Install

#### Download binary
Go to the [GitHub Releases](https://github.com/bahlo/discord-retention-bot/releases)
and download the binary for your architecture.

#### Docker
You can use the provided Docker image at
`docker.pkg.github.com/bahlo/discord-retention-bot/discord-retention-bot:1.0.0`, 
it only needs internet access and the provided environment variables 
(see [Configuration](#Configuration)).

#### Build from source

1. Clone the repository with 
   `git clone https://github.com/bahlo/discord-retention-bot`
2. Export the environment variables or write them to `.env` (see 
   [Configuration](#Configuration))
3. Run `cargo build --release` to build your binary to 
   `target/release/discord-retention-bot`

## Configuration

Configuration is happening via environment variables or an `.env` file:

* `RUST_LOG` defines the log level (I recommend setting this to `info`)
* `DISCORD_TOKEN` is the token of your Discord bot
* `CHANNEL_RETENTION` is a list of channel names and the duration to keep, separated by
  a comma. For example: `general:2w,random:4d` (currently the duration only 
  support s`d` and `w`, please open an issue if you need another one)

## Troubleshooting
### Why is it taking so long?
Discord might be rate-limiting you. This applications uses the single message
delete endpoint because [Bulk Delete Messages](https://discord.com/developers/docs/resources/channel#bulk-delete-messages) doesn't support messages older than 2 weeks. 
It might take a while the first time, but it will get faster.

## It's not deleting the messages of a channel
Make sure the bot has access to that channel in the Discord application and the 
following permissions:
* Read Text Channels & See Voice Channels
* Manage Messages
* Read Message History