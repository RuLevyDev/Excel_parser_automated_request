// utils/post_request.rs
use reqwest::blocking::Client;
use std::error::Error;

/// Sends a POST request with activity data.
///
/// # Arguments
/// * `actividad_json` - The activity data in JSON format.
/// * `endpoint` - The URL of the API endpoint.
///
/// # Returns
/// * `Result<String, Box<dyn Error>>` - The response from the server or an error.
pub fn post_request(actividad_json: &str, endpoint: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .body(actividad_json.to_string())
        .send()?;

    if response.status().is_success() {
        let body = response.text()?;
        Ok(body)
    } else {
        Err(format!("Error al enviar la actividad: {}", response.status()).into())
    }
}
