use chrono::{DateTime, Duration, Utc};
use reqwest::header::CONTENT_TYPE;
use cucumber::{given, then, when};
use std::collections::HashMap;
use url::{form_urlencoded};
use reqwest::StatusCode;
use rust_project::utils;
use serde_json::Value;
use crate::ApiWorld;
use log::info;
use std::env;

#[given(expr = "The API path is {string}")]
async fn set_api_url(world: &mut ApiWorld, path: String) {
    let api_base_url = env::var("API_BASE_URL").expect("API_BASE_URL must be set");
    let api_url = format!("{}{}", api_base_url, path);
    info!("Setting API URL: {}", api_url);
    world.api_url = api_url;
}

#[given("The API key is set")]
async fn set_api_key(world: &mut ApiWorld) {
    world.api_key = env::var("API_KEY").expect("API_KEY must be set");
    info!("API key is set: {}", world.api_key);
}

#[when("I send a GET request")]
async fn send_get_request(world: &mut ApiWorld) {
    info!("Sending GET request to: {}", world.api_url);

    // Create a client
    let client = reqwest::Client::new();

    // Perform GET request
    let response = client
        .get(&world.api_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("User-Agent", "MyRustBDDTest/1.0")
        .send()
        .await
        .expect("Failed to get response");

    world.response_status = Some(response.status());
    let response_body_text = response.text().await.expect("Failed to read response body");
    info!("Full Response body: {}", response_body_text);
    let response_body_json: Value = serde_json::from_str(&response_body_text).expect("Failed to convert to JSON string");

    // Extract the "result" node
    if let Some(result) = response_body_json.get("result") {

        // Convert the "result" node to a pretty JSON string
        let result_json = serde_json::to_string_pretty(result).expect("Failed to convert to JSON string");
        world.response_body = Option::from(result_json);
    }
}

#[then(expr = "The response status should be {int}")]
async fn check_response_status(world: &mut ApiWorld, expected_status: u16) {
    let expected_status = StatusCode::from_u16(expected_status).expect("Invalid status code");
    info!("Expected status: {}, Actual status: {:?}", expected_status, world.response_status);
    assert_eq!(world.response_status, Some(expected_status));
}

#[then(expr = "The response body should contain {string} key")]
async fn check_response_body(world: &mut ApiWorld, expected_key: String) {
    let response_body = world.response_body.as_ref().expect("No response body");
    let parsed_response_body: Value = serde_json::from_str(response_body).expect("JSON was not well-formatted");
    info!("Expected value: {}, Actual status: {:?}", expected_key, world.response_body);
    assert!(parsed_response_body.get(&expected_key).is_some(), "Key {} is missing", &expected_key);
}

#[then(expr = "The {string} node should contain {string} key with {string} value")]
async fn check_response_key_and_value(world: &mut ApiWorld, node: String, key: String, value: String) {
    let response_body = world.response_body.as_ref().expect("No response body");
    let parsed_response_body: Value = serde_json::from_str(response_body).expect("JSON was not well-formatted");
    let actual_wrapped_value = parsed_response_body[&node][&key].as_str();
    let actual_value = actual_wrapped_value.unwrap().trim_matches('"').to_string();
    assert_eq!(value, actual_value, "Expected value: {}, Actual value: {:?}", value, actual_value);
}

#[then(expr = "Server time and current time difference should be within tolerance")]
async fn check_time(world: &mut ApiWorld) {
    let response_body = world.response_body.as_ref().expect("No response body");
    let parsed_response_body: Value = serde_json::from_str(response_body).expect("JSON was not well-formatted");
    let server_date_time = DateTime::parse_from_rfc3339(&parsed_response_body["timestamp"].as_str().unwrap())
        .expect("Failed to parse server time")
        .with_timezone(&Utc);
    let current_time = Utc::now();
    info!("Current ISO 8601 date and time: {}", current_time);
    info!("Server date and time: {}", server_date_time);

    // Check that the server time is within 1 seconds of the current time
    let tolerance = Duration::seconds(1);
    let difference = (current_time - server_date_time).num_seconds().abs();
    info!("Time difference: {}", difference);
    assert!(difference <= tolerance.num_seconds(), "Server time is not within tolerance. Difference: {} seconds", difference);
}

#[when("I request open orders")]
async fn request_open_orders(world: &mut ApiWorld) {
    info!("Sending POST request to: {}", world.api_url);

    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let nonce = Utc::now().timestamp_millis().to_string();

    // Prepare the request data
    let mut data = HashMap::new();
    data.insert("nonce", nonce.as_str());
    data.insert("trades", "true");

    // Create API utils
    let api_signature = utils::sign::generate_api_sign("/0/private/OpenOrders", &nonce, &data);

    // URL encode the data
    let post_data: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(data.iter())
        .finish();

    // Build the request
    let client = reqwest::Client::new();

    let request = client
        .post(&world.api_url)
        .header("API-Key", &api_key)
        .header("API-Sign", api_signature)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded; charset=utf-8")
        .body(post_data.clone());

    // Convert the request builder to a request
    let request = request.build().expect("Failed to build request");

    info!("Request URL: {}", request.url());
    info!("Request Method: {}", request.method());
    info!("Request Headers: {:?}", request.headers());

    // Send the request and handle the response
    let response = client
        .execute(request)
        .await
        .expect("Failed to send request");

    world.response_status = Some(response.status());

    // Ensure the request was successful (HTTP status code 200)
    if response.status().is_success() {
        let response_body_text = response.text().await.expect("Failed to read response body");
        info!("Full Response body: {}", response_body_text);
        let response_body_json: Value = serde_json::from_str(&response_body_text).expect("Failed to convert to JSON string");
        // Extract the "result" node
        if let Some(result) = response_body_json.get("result") {

            // Convert the "result" node to a pretty JSON string
            let result_json = serde_json::to_string_pretty(result).expect("Failed to convert to JSON string");
            world.response_body = Option::from(result_json);
        }
    } else {
        // Print the error status code and message
        println!("Error: {}", response.status());
    }
}
