// Copyright 2024 the octopower authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use enphase_local::Envoy;
use eyre::Report;
use reqwest::Url;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Report> {
    pretty_env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("  {} <base URL> <auth token>", args[0]);
        exit(1);
    }
    let base_url = &args[1];
    let auth_token = &args[2];

    let envoy = Envoy::new(Url::parse(base_url)?, auth_token);
    println!("Home: {:#?}", envoy.home().await?);
    println!("Inventory: {:#?}", envoy.inventory(true).await?);
    println!("Production: {:#?}", envoy.production().await?);

    Ok(())
}
