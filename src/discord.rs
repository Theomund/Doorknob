// Doorknob - Artificial intelligence program written in Rust.
// Copyright (C) 2024 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::env;

use crate::chat::complete;
use crate::image::generate;

use poise::async_trait;
use poise::serenity_prelude as serenity;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use songbird::SerenityInit;
use tracing::{error, info};

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }
        None
    }
}

struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn chat(
    ctx: Context<'_>,
    #[description = "Query that is passed to the AI."] query: String,
) -> Result<(), Error> {
    let response = complete(query.as_str()).await?;
    ctx.reply(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Error retrieving guild ID.");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.reply("I'm not in a voice channel.").await?;
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        ctx.reply("I'm already deafened.").await?;
    } else {
        if let Err(why) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {why:?}")).await?;
        }

        ctx.reply("I'm now deafened.").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn draw(
    ctx: Context<'_>,
    #[description = "Query that is passed to the AI."] query: String,
) -> Result<(), Error> {
    let response = generate(query.as_str()).await?;
    for path in response {
        ctx.send(
            poise::CreateReply::default().attachment(serenity::CreateAttachment::path(path).await?),
        )
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Error retrieving guild from message.");
        let author_id = ctx.author().id;
        let channel_id = guild
            .voice_states
            .get(&author_id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let Some(connect_to) = channel_id else {
        ctx.reply("You're not in a voice channel.").await?;
        return Ok(());
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        ctx.reply("Joined voice channel.").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Error retrieving guild ID.");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild_id).await {
            ctx.reply(format!("Failed: {why:?}")).await?;
        }

        ctx.reply("Left voice channel.").await?;
    } else {
        ctx.reply("I'm not in a voice channel.").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Error retrieving guild ID.");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.reply("I'm not in a voice channel.").await?;
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        ctx.reply("I'm already muted.").await?;
    } else {
        if let Err(why) = handler.mute(true).await {
            ctx.reply(format!("Failed: {why:?}")).await?;
        }

        ctx.reply("I'm now muted.").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn undeafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Error retrieving guild ID.");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.reply("I'm not in a voice channel.").await?;
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        if let Err(why) = handler.deafen(false).await {
            ctx.reply(format!("Failed: {why:?}")).await?;
        }

        ctx.reply("I'm now undeafened.").await?;
    } else {
        ctx.reply("I'm already undeafened.").await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn unmute(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Error retrieving guild ID.");

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird client isn't initialized.")
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        ctx.reply("I'm not in a voice channel.").await?;
        return Ok(());
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        if let Err(why) = handler.mute(false).await {
            ctx.reply(format!("Failed: {why:?}")).await?;
        }

        ctx.reply("I'm now unmuted.").await?;
    } else {
        ctx.reply("I'm already unmuted.").await?;
    }

    Ok(())
}

pub async fn initialize() {
    let token = env::var("DISCORD_TOKEN").expect("Expected the token in an environment variable.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                chat(),
                deafen(),
                draw(),
                join(),
                leave(),
                mute(),
                ping(),
                undeafen(),
                unmute(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client.");
    if let Err(why) = client.start().await {
        info!("Client error: {why:?}");
    }
    info!("Initialized the Discord module.");
}
