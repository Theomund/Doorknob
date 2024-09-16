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

use crate::discord::Error;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

pub async fn complete(query: &str) -> Result<String, Error> {
    let client = Client::new();
    let prompt = ChatCompletionRequestSystemMessageArgs::default()
        .content("Your name is Doorknob. You're a conversational chatbot in a Discord server.")
        .build()
        .unwrap()
        .into();
    let message = ChatCompletionRequestUserMessageArgs::default()
        .content(query)
        .build()?
        .into();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o")
        .messages(vec![prompt, message])
        .build()?;
    let response = client.chat().create(request).await?;
    let choice = response.choices.first().unwrap();
    let content = choice.message.content.clone().unwrap();
    Ok(content)
}
