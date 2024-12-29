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
    dotenv::dotenv().expect("failed to load .env");
    let token = env::var("DISCORD_TOKEN").expect("expected DISCORD_TOKEN in .env");

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("error initialising client");

    let http = Arc::clone(&client.cache_and_http.http);

    listServersAndUsers(&http).await;

    client.shard_manager.lock().await.shutdown_all().await;
}

async fn listServersAndUsers(http: &Arc<Http>) {
    let guilds = http
        .get_guilds(None, Some(100))
        .await
        .expect("failed to pull guilds/servers");

    println!("select a server (number):");
    for (i, guild) in guilds.iter().enumerate() {
        println!("{}: {}", i + 1, guild.name);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");

    let serverIndex: usize = input
        .trim()
        .parse::<usize>()
        .expect("invalid input") - 1;

    if serverIndex >= guilds.len() {
        println!("invalid selection - exiting program");
        return;
    }

    let selectedGuildId = guilds[serverIndex].id;

    let members = selectedGuildId
        .members(http, None, None)
        .await
        .expect("failed to pull members");

    let mut mutuals = Vec::new();

    let botGuilds = guilds.iter().map(|guild| guild.id).collect::<Vec<_>>();

    for member in members {
        let user = &member.user;

        let userGuilds = user
            .guilds(http)
            .await
            .expect("Failed to fetch user guilds");

        let mutualGuilds: Vec<String> = userGuilds
            .iter()
            .filter(|guild| botGuilds.contains(&guild.id))
            .map(|guild| guild.name.clone())
            .collect();

        if !mutualGuilds.is_empty() {
            mutuals.push((user.name.clone(), mutualGuilds));
        }
    }

    println!("users with mutual servers:");
    for (username, mutualServers) in mutuals {
        println!("user: {}, mutual servers: {}", username, mutualServers.join(", "));
    }
}
