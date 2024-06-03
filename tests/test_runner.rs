use simplelog::{CombinedLogger, WriteLogger, Config};
use reqwest::{StatusCode};
use log::{LevelFilter};
use cucumber::{World};
use std::fs::File;

mod step_definitions;

#[derive(Debug, Default, World)]
struct ApiWorld {
    api_url: String,
    api_key: String,
    response_status: Option<StatusCode>,
    response_body: Option<String>,
}

#[tokio::main]
async fn main() {
    // Set up logging to a file
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("output/test_execution.log").unwrap()),
    ])
        .unwrap();

    ApiWorld::cucumber()
        .max_concurrent_scenarios(1)
        .run("tests/features")
        .await;
}