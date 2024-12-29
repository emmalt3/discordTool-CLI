use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::http::Http;
use serenity::model::prelude::*;
use std::env;
use std::sync::Arc;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env file");

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let http = Arc::clone(&client.cache_and_http.http);

    list_servers_and_users(&http).await;

    client.shard_manager.lock().await.shutdown_all().await;
}

async fn list_servers_and_users(http: &Arc<Http>) {
    let guilds = http
        .get_guilds(None, Some(100))
        .await
        .expect("Failed to fetch guilds");

    println!("Select a server by number:");
    for (i, guild) in guilds.iter().enumerate() {
        println!("{}: {}", i + 1, guild.name);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let server_index: usize = input
        .trim()
        .parse::<usize>()
        .expect("Invalid input") - 1;

    if server_index >= guilds.len() {
        println!("Invalid selection. Exiting.");
        return;
    }

    let selected_guild_id = guilds[server_index].id;

    let members = selected_guild_id
        .members(http, None, None)
        .await
        .expect("Failed to fetch members");

    let mut mutuals = Vec::new();

    for member in members {
        let user = &member.user;

        let mut mutual_guilds = Vec::new();

        for guild in &guilds {
            if guild.id == selected_guild_id {
                continue;
            }

            let member = guild
                .id
                .member(http, user.id)
                .await
                .ok();

            if member.is_some() {
                mutual_guilds.push(guild.name.clone());
            }
        }

        if !mutual_guilds.is_empty() {
            mutuals.push((user.name.clone(), mutual_guilds));
        }
    }

    println!("Users with mutual servers:");
    for (username, mutual_servers) in mutuals {
        println!("User: {}, Mutual Servers: {}", username, mutual_servers.join(", "));
    }
}
