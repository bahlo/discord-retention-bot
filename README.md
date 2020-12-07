# discord-retention-bot
A discord bot that allows you to set a retention for different channels (similar to Slack)

## Install
1. Create a Discord Application in the 
   [Discord Developer Portal](https://discord.com/developers/applications)
2. Copy the client id from the "General information" tab
3. Go to <https://discord.com/oauth2/authorize?client_id=$CLIENT_ID&scope=bot&permissions=74752>
   and add your Bot to your discord server.

74752 is the bot permission bitmask for the following permissions:
* View Channels
* Manage Messages
* Read Message History