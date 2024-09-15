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

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::Result as SerenityResult;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use songbird::SerenityInit;
use tracing::{error, info};

struct Handler;

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

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name != "Doorknob" {
            match msg.content.as_str() {
                "!deafen" => {
                    let guild_id = msg.guild_id.expect("Error retrieving guild ID.");

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client isn't initialized.")
                        .clone();

                    let Some(handler_lock) = manager.get(guild_id) else {
                        check_msg(msg.reply(ctx, "I'm not in a voice channel").await);
                        return;
                    };

                    let mut handler = handler_lock.lock().await;

                    if handler.is_deaf() {
                        check_msg(msg.channel_id.say(&ctx.http, "I'm already deafened.").await);
                    } else {
                        if let Err(why) = handler.deafen(true).await {
                            check_msg(
                                msg.channel_id
                                    .say(&ctx.http, format!("Failed: {why:?}"))
                                    .await,
                            );
                        }

                        check_msg(msg.channel_id.say(&ctx.http, "I'm now deafened.").await);
                    }
                }
                "!join" => {
                    let (guild_id, channel_id) = {
                        let guild = msg
                            .guild(&ctx.cache)
                            .expect("Error retrieving guild from message.");
                        let channel_id = guild
                            .voice_states
                            .get(&msg.author.id)
                            .and_then(|voice_state| voice_state.channel_id);

                        (guild.id, channel_id)
                    };

                    let Some(connect_to) = channel_id else {
                        check_msg(msg.reply(ctx, "You're not in a voice channel.").await);
                        return;
                    };

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client is not initialized.")
                        .clone();

                    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
                        let mut handler = handler_lock.lock().await;
                        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
                        check_msg(msg.channel_id.say(&ctx.http, "Joined voice channel.").await);
                    }
                }
                "!leave" => {
                    let guild_id = msg.guild_id.expect("Error retrieving guild ID.");

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client isn't initialized.")
                        .clone();

                    let has_handler = manager.get(guild_id).is_some();

                    if has_handler {
                        if let Err(why) = manager.remove(guild_id).await {
                            check_msg(
                                msg.channel_id
                                    .say(&ctx.http, format!("Failed: {why:?}"))
                                    .await,
                            );
                        }

                        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel.").await);
                    } else {
                        check_msg(msg.reply(ctx, "I'm not in a voice channel.").await);
                    }
                }
                "!mute" => {
                    let guild_id = msg.guild_id.expect("Error retrieving guild ID.");

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client isn't initialized.")
                        .clone();

                    let Some(handler_lock) = manager.get(guild_id) else {
                        check_msg(msg.reply(ctx, "I'm not in a voice channel").await);
                        return;
                    };

                    let mut handler = handler_lock.lock().await;

                    if handler.is_mute() {
                        check_msg(msg.reply(ctx, "I'm already muted.").await);
                    } else {
                        if let Err(why) = handler.mute(true).await {
                            check_msg(
                                msg.channel_id
                                    .say(&ctx.http, format!("Failed: {why:?}"))
                                    .await,
                            );
                        }

                        check_msg(msg.channel_id.say(&ctx.http, "I'm now muted.").await);
                    }
                }
                "!ping" => {
                    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);
                }
                "!undeafen" => {
                    let guild_id = msg.guild_id.expect("Error retrieving guild ID.");

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client isn't initialized.")
                        .clone();

                    let Some(handler_lock) = manager.get(guild_id) else {
                        check_msg(msg.reply(ctx, "I'm not in a voice channel").await);
                        return;
                    };

                    let mut handler = handler_lock.lock().await;

                    if handler.is_deaf() {
                        if let Err(why) = handler.deafen(false).await {
                            check_msg(
                                msg.channel_id
                                    .say(&ctx.http, format!("Failed: {why:?}"))
                                    .await,
                            );
                        }

                        check_msg(msg.channel_id.say(&ctx.http, "I'm now undeafened.").await);
                    } else {
                        check_msg(msg.reply(ctx, "I'm already undeafened.").await);
                    }
                }
                "!unmute" => {
                    let guild_id = msg.guild_id.expect("Error retrieving guild ID.");

                    let manager = songbird::get(&ctx)
                        .await
                        .expect("Songbird client isn't initialized.")
                        .clone();

                    let Some(handler_lock) = manager.get(guild_id) else {
                        check_msg(msg.reply(ctx, "I'm not in a voice channel").await);
                        return;
                    };

                    let mut handler = handler_lock.lock().await;

                    if handler.is_mute() {
                        if let Err(why) = handler.mute(false).await {
                            check_msg(
                                msg.channel_id
                                    .say(&ctx.http, format!("Failed: {why:?}"))
                                    .await,
                            );
                        }

                        check_msg(msg.channel_id.say(&ctx.http, "I'm now unmuted.").await);
                    } else {
                        check_msg(msg.reply(ctx, "I'm already unmuted.").await);
                    }
                }
                command if command.starts_with('!') => {
                    check_msg(
                        msg.reply(ctx, format!("Sorry, I don't know the `{command}` command."))
                            .await,
                    );
                }
                _ => {}
            }
        }
    }
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        error!("Error sending message: {why:?}");
    }
}

pub async fn initialize() {
    let token = env::var("DISCORD_TOKEN").expect("Expected the token in an environment variable.");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client.");
    if let Err(why) = client.start().await {
        info!("Client error: {why:?}");
    }
    info!("Initialized the Discord module.");
}
