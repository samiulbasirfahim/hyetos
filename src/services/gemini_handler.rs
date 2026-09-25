use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

pub async fn ask_gemini(
    client: &Client,
    system_instruction: &str,
    content: &str,
) -> Result<String, Box<dyn Error>> {
    let config = crate::Config::get();
    let url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

    let body = json!({
        "system_instruction" : {
            "parts": [{"text": system_instruction}]
        },
        "contents": [
            {
                "role": "user",
                "parts": [{"text": content}]
            }
        ]
    });

    let res: Value = client
        .post(url)
        .header("x-goog-api-key", config.gemini_api_key.trim())
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let text = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(text)
}
