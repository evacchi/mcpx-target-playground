use serde_json::Value;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
}

impl TryFrom<&Value> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let api_key = value["api-key"]
            .as_str()
            .ok_or("Missing or invalid api-key")?
            .to_string();
        Ok(Config { api_key })
    }
}

pub async fn api_key_fetch(input: &Value, config: &Config) -> Result<Value, Box<dyn Error>> {
    let url = input["url"]
        .as_str()
        .ok_or("Missing or invalid url parameter")?;
    
    // Replace $APIKEY with the actual API key
    let url = url.replace("$APIKEY", &config.api_key);
    
    // Fetch the content
    let response = reqwest::get(&url).await?;
    let text = response.text().await?;
    
    // Return the response as a JSON string
    Ok(Value::String(text))
}
