use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Input {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "api-key")]
    api_key: String,
}

#[plugin_fn]
pub fn api_key_fetch(Json(input): Json<Input>) -> FnResult<Json<String>> {
    // Get the configuration
    let config: Config = config_get()?;
    
    // Replace $APIKEY with the actual key
    let url = input.url.replace("$APIKEY", &config.api_key);
    
    // Fetch the content
    let response = http::request(
        http::Request::new("GET", &url)
            .with_header("User-Agent", "Extism Plugin")
    )?.into_string()?;
    
    Ok(Json(response))
}
