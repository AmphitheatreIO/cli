// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::StripPrefixError;

use amp_common::{client, filesystem};
pub use anyhow::*;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Errors>;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Invalid configuration path")]
    InvalidConfigPath(#[source] confy::ConfyError),

    #[error("Failed to load configuration")]
    FailedLoadConfiguration(#[source] anyhow::Error),

    #[error("Current context not found, please use `amp context` for help")]
    NotFoundCurrentContext,

    #[error("Client error: {0}")]
    ClientError(client::ClientError),

    #[error("Failed to load manifest: {0}")]
    FailedLoadManifest(String),

    #[error("Could not find `.amp.toml` in current directory or any parent directory")]
    NotFoundManifest(#[source] filesystem::Error),

    #[error("Failed to delete playbook: {0}")]
    FailedDeletePlaybook(String),

    #[error("Failed to delete context: {0}")]
    FailedDeleteContext(anyhow::Error),

    #[error("Not found context: {0}")]
    NotFoundContext(String),

    #[error("Failed to save configuration")]
    FailedSaveConfiguration(anyhow::Error),

    #[error("Failed to serialize toml")]
    TomlSerializeError(toml::ser::Error),

    #[error("Failed to save manifest: {0}")]
    FailedSaveManifest(std::io::Error),

    #[error("Invalid manifest: {0}")]
    InvalidManifest(toml::de::Error),

    #[error("Failed to create playbook: {0}")]
    FailedCreatePlaybook(client::ClientError),

    #[error("Failed to finish tar: {0}")]
    FailedFinishTar(std::io::Error),

    #[error("Walk directory error: {0}")]
    WalkError(ignore::Error),

    #[error("Failed to strip prefix: {0}")]
    FailedStripPrefix(StripPrefixError),

    #[error("Failed to append path: {0}")]
    FailedAppendPath(std::io::Error),
}