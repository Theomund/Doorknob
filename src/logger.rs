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
use std::str::FromStr;

use crate::types::Error;

use tracing::{info, Level};

pub fn initialize() -> Result<(), Error> {
    let level = Level::from_str(env::var("LOG_LEVEL")?.as_str())?;
    tracing_subscriber::fmt().with_max_level(level).init();
    info!("Initialized the logger module.");
    Ok(())
}
