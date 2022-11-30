// Copyright 2022 The Amphitheatre Authors.
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

use client::client::Client;
use client::playbooks::PlaybookPayload;
use errors::Result;
use std::{thread, time::Duration};

pub fn run() -> Result<()> {
    // Handle signal.
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Validate

    // Create playbook from this Character
    let client = Client::new(String::from("AUTH_TOKEN"));
    let payload = PlaybookPayload {
        title: "test".to_string(),
        description: "".to_string(),
        lead: "https://github.com/amphitheatre-app/amp-example-go".to_string(),
    };
    let response = client.playbooks().create(payload);
    if let Err(e) = response {
        eprintln!("Error: Could not create the playbook ({})", e);
        std::process::exit(1);
    }

    let playbook = response.unwrap().data.unwrap();

    // // Sync the source to remote Dev Container
    // if let Err(e) = sync(".".to_string(), src(&playbook.lead())) {
    //     eprintln!("Error: Could not sync the sources ({})", e);
    //     std::process::exit(1);
    // }

    // Run
    if let Err(e) = client.playbooks().start(playbook.id) {
        eprintln!(
            "Error: Could not start playbook #{} ({})",
            &playbook.title, e
        );
        std::process::exit(1);
    }

    println!("Visit: http://{}.amphitheatre.app", &playbook.id);

    // Read event stream looply.
    loop {
        let event = client.playbooks().events(playbook.id);
        println!("Received event: {}", event);

        thread::sleep(Duration::from_secs(2));
    }
}