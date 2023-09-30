#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! reqwest = {version="0.11.20", features=["json"]}
//! tokio = {version="1.32.0", features=["full"]}
//!serde = {version="1.0.188", features=["derive"]}
//! http = "0.2.9"
//! ```

use http::{HeaderName, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use std::str::FromStr;
#[derive(Debug, Deserialize)]
pub struct MachineResponse {
    id: String,
}
#[derive(Debug, Deserialize)]
pub struct Success {
    result: String,
}
#[tokio::main]
async fn main() {
    let fly_token = std::env::var("FLY_TOKEN").unwrap();
    let client = Client::new();
    let res = client
        .get("https://api.machines.dev/v1/apps/benwis-leptos/machines")
        .bearer_auth(fly_token)
        .send()
        .await
        .unwrap();
    let json = res.json::<Vec<MachineResponse>>().await.unwrap();
    for item in json {
        println! {"{}",item.id}
        let res = client
            .get("https://benw.is/api/refetch")
            .query(&[("password", "spatulahorsepotato")])
            .header(
                HeaderName::from_str("fly-force-instance-id").unwrap(),
                HeaderValue::from_str(&item.id).unwrap(),
            )
            .header(
                http::header::ACCEPT,
                HeaderValue::from_str("application/json").unwrap(),
            )
            .send()
            .await
            .unwrap();
        println!("{:#?}", res.json::<Success>().await.unwrap())
    }
}
