// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use std::sync::Arc;

use amp_common::config::Cluster;
use clap::Args;
use tabled::settings::Style;
use tabled::Tabled;

use crate::context::Context;
use crate::errors::{Errors, Result};

/// List all available contexts
#[derive(Args, Debug)]
#[command(after_help = crate::cmd::cli::AFTER_HELP_STRING)]
pub struct Cli {}

impl Cli {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let configuration = ctx.configuration.read().await;
        let context = configuration.context.as_ref().ok_or(Errors::NotFoundContexts)?;

        let mut table = Vec::new();
        for (name, cluster) in context.iter() {
            let mut row = ContextTable::from(cluster);
            row.name.clone_from(name);
            if let Some((current, _)) = &context.current() {
                row.default = name.eq(current);
            }
            table.push(row);
        }
        println!("{}", tabled::Table::new(table).with(Style::modern()));

        Ok(())
    }
}

#[derive(Tabled)]
struct ContextTable {
    name: String,
    title: String,
    server: String,
    default: bool,
}

impl From<&Cluster> for ContextTable {
    fn from(ctx: &Cluster) -> Self {
        Self { name: String::new(), title: ctx.title.clone(), server: ctx.server.clone(), default: false }
    }
}
