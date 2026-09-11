use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

pub async fn ask_gemini(client: &Client, prompt: &str) -> Result<String, Box<dyn Error>> {
    let config = crate::Config::get();
    let model_name = "gemini-2.5-flash";
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        config.gemini_api_key.trim()
    );

    let res: Value = client
        .post(&url)
        .json(&json!({"contents": [{"parts": [{"text": prompt}]}]}))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let text = res["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    println!("[GEMINI] Prompt: {}", prompt);
    println!("[GEMINI] Response: {}", text);

    Ok(text)
}
