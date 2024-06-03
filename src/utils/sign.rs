use std::collections::HashMap;
use url::form_urlencoded;
use std::env;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512, Digest};
use base64::{Engine};
use base64::engine::general_purpose;
use sha2::digest::KeyInit;

// Alias for HMAC-SHA512
type HmacSha512 = Hmac<Sha512>;

pub fn generate_api_sign(url_path: &str, nonce: &str, data: &HashMap<&str, &str>) -> String {

    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set");
    let secret_decoded = general_purpose::STANDARD.decode(api_secret).unwrap();

    // URL encode the data
    let url_data: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(data.iter())
        .finish();

    // Concatenate nonce and post_data, then convert to bytes
    let concatenated = format!("{}{}", nonce, url_data);
    let encoded = concatenated.as_bytes();

    // Calculate SHA-256 hash of the encoded data
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    let sha256_digest = hasher.finalize();

    // Concatenate the URL path and the SHA-256 digest
    let mut message = Vec::new();
    message.extend_from_slice(&url_path.as_bytes());
    message.extend_from_slice(&sha256_digest);

    // Create HMAC-SHA512 instance and calculate the MAC
    let mut mac = HmacSha512::new_from_slice(&secret_decoded)
        .expect("HMAC can take key of any size");
    mac.update(&message);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    // Base64 encode the result
    let sigdigest = general_purpose::STANDARD.encode(code_bytes);

    sigdigest
}