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

use clap::{Arg, ArgMatches, Command};

pub fn build() -> Command<'static> {
    Command::new("test")
        .about("Run tests against your built application images")
        .args(&[
            Arg::new("assume-yes").long("assume-yes").takes_value(false).help("If true, amp will skip yes/no confirmation from the user and default to yes"),
            Arg::new("build-artifacts").short('a').long("build-artifacts").takes_value(true).help("File containing build result from a previous 'amp build --file-output'"),
            Arg::new("config").short('c').long("config").default_value("$HOME/.amp/config").help("File for global configurations"),
            Arg::new("filename").short('f').long("filename").default_value(".amp.yaml").help("Path or URL to the Amphitheatre config file"),
            Arg::new("images").short('i').long("images").takes_value(true).help("A list of pre-built images to deploy, either tagged images or NAME=TAG pairs"),
            Arg::new("module").short('m').long("module").default_value("[]").help("Filter Amphitheatre configs to only the provided named modules"),
            Arg::new("profile").short('p').long("profile").default_value("[]").help("Activate profiles by name (prefixed with `-` to disable a profile)"),
            Arg::new("profile-auto-activation").long("profile-auto-activation").takes_value(false).help("Set to false to disable profile auto activation"),
            Arg::new("propagate-profiles").long("propagate-profiles").takes_value(false)
                .help("Setting '--propagate-profiles=false' disables propagating profiles set by the '--profile' \
                    flag across config dependencies. This mean that only profiles defined directly in the\
                    target '.amp.yaml' file are activated."),
            Arg::new("remote-cache-dir").long("remote-cache-dir").default_value("$HOME/.amp/repos").help("Specify the location of the git repositories cache"),
            Arg::new("rpc-http-port").long("rpc-http-port").takes_value(true).help("tcp port to expose the Amphitheatre API over HTTP REST"),
            Arg::new("rpc-port").long("rpc-port").takes_value(true).help("tcp port to expose the Amphitheatre API over gRPC"),
            Arg::new("sync-remote-cache").long("sync-remote-cache").default_value("always")
            .help("Controls how Amphitheatre manages the remote config cache (see `remote-cache-dir`). \
                One of `always` (default), `missing`, or `never`. `always` syncs remote repositories \
                to latest on access. `missing` only clones remote repositories if they do not exist \
                locally. `never` means the user takes responsibility for updating remote repositories."),
            Arg::new("wait-for-connection").long("wait-for-connection").takes_value(false)
                .help("Blocks ending execution of amp until the /v2/events gRPC/HTTP endpoint is hit"),
        ])
        .after_help("Use \"amp options\" for a list of global command-line options (applies to all commands).")
}

pub fn execute(args: &ArgMatches) {
    todo!()
}