# discord-retention-bot 
![CI](https://github.com/bahlo/discord-retention-bot/workflows/CI/badge.svg)
![Audit](https://github.com/bahlo/discord-retention-bot/workflows/Audit/badge.svg)

A discord bot that allows you to set a retention for different channels (similar 
to Slack).

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
`docker.pkg.github.com/bahlo/discord-retention-bot/discord-retention-bot:1.0.0`.

#### Install with cargo
Run `cargo install discord-retention-bot` to install the lates version from 
[crates.io](https://crates.io).

#### Build from source

1. Clone the repository with 
   `git clone https://github.com/bahlo/discord-retention-bot`
2. Run `cargo build --release` to build your binary to 
   `target/release/discord-retention-bot`

## Configuration

Configuration is happening via environment variables or an `.env` file:

* `RUST_LOG` defines the log level (I recommend setting this to 
  `discord-retention-bot=info`)
* `DISCORD_TOKEN` is the token of your Discord bot
* `CHANNEL_RETENTION` is a list of channel names and the duration after which
  messages should be deleted, separated by a comma. 
  Example: `general:2w,random:4d` 
  (currently the duration only supports `h`, `d` and `w`, please open an issue 
  if you need another one). Please note that this applies to all guilds your
  bot is added to
* `DELETE_PINNED` can be set to `true` or `false` (default). If set to `true`, 
  pinned messages will also be deleted

## Troubleshooting
### Why is it taking so long?
Discord might be rate-limiting you. This applications uses the single message
delete endpoint because [Bulk Delete Messages](https://discord.com/developers/docs/resources/channel#bulk-delete-messages) doesn't support messages older than 2 weeks. 
It might take a while the first time, but it will get faster.

### It's not deleting the messages of a channel
Make sure the bot has access to that channel in the Discord application and the 
following permissions:
* Read Text Channels & See Voice Channels
* Manage Messages
* Read Message History

## Built on the shoulders of giants
Check the Cargo.toml for all packages used in this project. I just want to 
highlight [serenity](https://github.com/serenity-rs/serenity), a great library
for interacting with the Discord API.