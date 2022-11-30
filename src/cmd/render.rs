// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Args;
use errors::Result;

/// Perform all image builds, and output rendered Kubernetes manifests
#[derive(Args, Debug)]
#[command(after_help = super::cli::AFTER_HELP_STRING)]
pub struct Cli {
    /// If true, amp will skip yes/no confirmation from the user
    #[arg(long, action = clap::ArgAction::Set, default_value = "true")]
    assume_yes: bool,

    /// Path or URL to the Amphitheatre config file
    #[arg(short, long, default_value = ".amp.toml")]
    filename: Option<String>,

    /// Show the build logs and output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    loud: bool,

    /// File to write the changed config (instead of standard output)
    #[arg(short, long)]
    output: Option<String>,

    /// Activate profiles by name (prefixed with `-` to disable a profile)
    #[arg(short, long, default_value = "[]")]
    profile: Option<Vec<String>>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
