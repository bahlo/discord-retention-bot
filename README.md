# discord-retention-bot [![CI](https://github.com/bahlo/discord-retention-bot/workflows/CI/badge.svg)](https://github.com/bahlo/discord-retention-bot/actions?query=workflow%3ACI) [![Audit](https://github.com/bahlo/discord-retention-bot/workflows/Audit/badge.svg)](https://github.com/bahlo/discord-retention-bot/actions?query=workflow%3AAudit)

A bot that allows you to set a message retention for individual Discord text 
channels.

## Table of contents
* [Features](#features)
* [Preparation](#preparation)
* [Installation](#installation)
* [Configuration](#configuration)
* [Troubleshooting](#troubleshooting)
* [Integration tests](#integration-tests)

## Features
* Automatically delete messages that are older than a configured time
* Don't delete pinned messages until configured otherwise
* Multi channel configuration (e.g. keep messages `#general` for two weeks, but 
  `#random` for one day)
* Default configuration for all channels without definend retention

## Preparation
Before running your bot you need to create it on Discord:

1. Create a Discord Application in the 
   [Discord Developer Portal](https://discord.com/developers/applications)
2. Go to `Bot` and click `Add Bot` and make sure to uncheck `Public bot`
3. Copy the `CLIENT ID` from the `General Information` tab
4. Go to <https://discord.com/oauth2/authorize?client_id=$CLIENT_ID&scope=bot&permissions=74752>
   and add your bot to your Discord server

### Wait, what does 74752 mean?
74752 is the bitmask for the following permissions:

* View Channels
* Manage Messages
* Read Message History

You can verify this by checking these in the Bot Permissions mask on your bots 
page.

## Installation

### Download binary
Go to the [GitHub Releases](https://github.com/bahlo/discord-retention-bot/releases)
and download the binary for your architecture.

### Docker
You can use the provided Docker image at
`docker.pkg.github.com/bahlo/discord-retention-bot/discord-retention-bot:1.0.1`.

### Cargo
Run `cargo install discord-retention-bot` to install the latest version from 
[crates.io](https://crates.io).

### Build from source
1. Clone the repository with 
   `git clone https://github.com/bahlo/discord-retention-bot`
2. Run `cargo build --release` to build your binary to 
   `target/release/discord-retention-bot`

## Configuration

Configure your bot via environment variables (optionally in an `.env` file).

### `RUST_LOG` 
Tihs defines the log level. I recommend setting this to 
`discord-retention-bot=info` for normal usage.

### `DISCORD_TOKEN` 
The token of your Discord bot. Get it from the 
[Discord Developer Portal](https://discord.com/developers) by going to your
application → Bot and copying the token.

### `DELETE_PINNED` 
Can be set to `true` or `false`. If set to `true`, pinned messages 
will also be deleted. Defaults to `false`.

### `CHANNEL_RETENTION` 
A list of channel names and the duration after which messages should be deleted, 
separated by a comma. You can also configure `*` to match all unconfigured 
channnels.
The duration is a number followed by one of `h` (hours), `d` (days), and `w` 
(weeks).
Please note that this configuration applies to all guilds your bot is added to.

#### Example
`general:2w,random:4d,*:4w` will result in messages being deleted in
* `general`: after two weeks
* `random`: after four days
* every other channel after four weeks

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

## Integration tests
To run integration tests you need to create a bot (see 
[Preparation](#Preparation)), but with the bitmask 76816, which translates to:

* Manage Channels
* View Channels
* Send Messages
* Manage Messages
* Read Message History

Export the bot token to `INTEGRATION_DISCORD_TOKEN` and run `cargo test -- --ignored` to run the integration tests.
