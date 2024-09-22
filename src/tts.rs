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

use crate::types::Error;

use async_openai::{
    types::{CreateSpeechRequestArgs, SpeechModel, Voice},
    Client,
};

pub async fn generate(text: &str) -> Result<(), Error> {
    let client = Client::new();
    let request = CreateSpeechRequestArgs::default()
        .input(text)
        .model(SpeechModel::Tts1Hd)
        .voice(Voice::Echo)
        .build()?;
    let response = client.audio().speech(request).await?;
    response.save("./data/speech.mp3").await?;
    Ok(())
}
