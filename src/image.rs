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

use std::path::PathBuf;

use crate::types::Error;

use async_openai::{
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
    Client,
};

pub async fn generate(query: &str) -> Result<Vec<PathBuf>, Error> {
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .n(1)
        .model(ImageModel::DallE3)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S1024x1024)
        .prompt(query)
        .build()?;
    let response = client.images().create(request).await?;
    let paths = response.save("./data").await?;
    Ok(paths)
}
