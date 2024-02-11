mod commands;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot {
    client: reqwest::Client,
    discord_guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = GuildId::set_commands(self.discord_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("Replies with pong")
                })
                .create_application_command(|command| {
                    command
                        .name("socials")
                        .description("https://bcnrust.github.io/")
                })
        })
        .await
        .unwrap();
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
