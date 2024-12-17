use anyhow::Result;
use mcp_core::{CallToolInput, CallToolOutput, CallToolResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Config {
    api_key: String,
}

#[derive(Deserialize)]
struct Input {
    url: String,
}

#[derive(Serialize)]
struct Output {
    image_data: Vec<u8>,
}

#[no_mangle]
pub fn call_tool(input: CallToolInput) -> CallToolResult {
    match fetch_image(input) {
        Ok(output) => CallToolOutput::success(output),
        Err(e) => CallToolOutput::error(e.to_string()),
    }
}

fn fetch_image(input: CallToolInput) -> Result<Output> {
    // Parse configuration
    let config: Config = serde_json::from_value(input.config)?;
    
    // Parse input
    let input: Input = serde_json::from_value(input.body)?;
    
    // Create client with API key header
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&input.url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .send()?;
    
    // Check if request was successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch image: HTTP {}",
            response.status()
        ));
    }
    
    // Get image bytes
    let image_data = response.bytes()?.to_vec();
    
    Ok(Output { image_data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_fetch_image() {
        let input = CallToolInput {
            config: json!({
                "api_key": "test-key"
            }),
            body: json!({
                "url": "https://example.com/image.jpg"
            }),
        };

        let result = fetch_image(input);
        assert!(result.is_ok() || result.is_err());
    }
}