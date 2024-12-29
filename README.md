# CLI Discord Tool for Checking Mutual Servers Between Users:

This project is a command line tool that interacts with the Discord API using a bot. It allows you to list servers and users in a server, and check for mutual servers between users. The tool is built using the Serenity Rust library in combination with Tokio for asynchronous integration.

## Features:
* List all servers your bot is in
* Allow the user to select a server
* Retrieve users in the selected server
* Check for mutual servers between users (across multiple servers)
* CLI for ease of use

## Prerequisits:

### 1. Install Rust
The bot is written in the Rust Programming Language - so you'll need to have this installed and up-to date!

**Bash / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh/rustup.rs | sh
```

**Powershell / Windows:**
```powershell
Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile rustup-init.exe; .\rustup-init.exe
```

### 2. Set up a Discord Bot
Before running this bot, you need to create a Discord Bot Application in [Discord’s Developer Portal](https://discord.com/developers/applications) and copy your bot token.

**Steps:**
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click `New Applicaton`
3. Give your bot a name
4. In the left sidebar, go to `Bot` and click `Add Bot`
5. Under your `Bot` settings, click `Copy` to get your bots Token. This will be used to authenticate your bot with Discord.

**Setting Bot Permissions:**
In the `OAuth2` section in the left sidebar, generate an invite URL for your bot by selecting:
1. `Bot` under `Scopes`
2. Select `View Channels` under `Bot Permissions`
3. Invite your bot to your server using this URL

### 3. Clone the Repository
Now that your Discord Bot is set up, you can clone this program to your local computer:

**Bash / Linux:**
```bash
git clone https://github.com/emmalt3/discordTool-CLI.git
```

**Powershell / Windows:**
```powershell
git clone https://github.com/emmalt3/discordTool-CLI.git
```

### 4. Change Directory into the Bots Location
Ensures we are in the root of the cloned repository

**Bash / Linux:**
```bash
cd discordTool-CLI
```

**Powershell / Windows:**
```powershell
cd C:\path\to\discordTool-CLI
```

### 5. Create a `.env` File
The bot requires your Discord Bot Token to run. Store this value in your `.env` file, in the root of the cloned repository.

**Bash / Linux:**
```bash
echo "DISCORD_TOKEN=replaceWithYourBotToken" > .env
```

**Powershell / Windows:**
```powershell
echo "DISCORD_TOKEN=replaceWithYourBotToken" > .env
```

Be sure to replace `replaceWithYourBotToken` with your Discord Bot Token that you coppied from earlier!

### 6. Installing Dependancies and Compiling the Project
Run these commands in the root of the cloned repository to install the dependancies and compile the project.

**Bash / Linux:**
```bash
cargo build
```

**Powershell / Windows:**
```powershell
cargo build
```

### 7. Run the Project
You should now be able to run the project from the CLI using the following command.

**Bash / Linux:**
```bash
cargo run
```

**Powershell / Windows:**
```powershell
cargo run
```

## Using the Bot:
### 1. Select a Server
When you run the bot, you will be prompted to select a server from the list of servers that the bot is in. You can input the number corresponding to the server you want to interact with.

### 2. Lists Users and Mutual Servers

When a server has been selected, the bot pulls all members of that server and checks which other servers those members are in. It will then list the members who share mutual servers with others.



