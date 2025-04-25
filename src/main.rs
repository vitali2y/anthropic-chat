// Simple Rust CLI chat with Anthropic AI

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    error::Error,
    io::{self, Read},
};

#[derive(Debug, Serialize, Deserialize)]
struct CompletionResponse {
    completion: String,
    stop_reason: String,
    model: String,
    log_id: String,
}

async fn get_completion(api_key: &str, prompt: &str) -> Result<CompletionResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.anthropic.com/v1/messages";

    let request_body = json!({
        "messages": [ {"role": "user", "content": prompt} ],
        "model": "claude-3-7-sonnet-20250219",
        "max_tokens": 200,
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01") // check Anthropic documentation for latest
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let completion_response: CompletionResponse = response.json().await?;
        Ok(completion_response)
    } else {
        let status = response.status();
        let body = response.text().await?;
        Err(format!(
            "API request failed with \"{}\" status and body: {}",
            status, body
        )
        .into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY env var not set!");

    let args: Vec<String> = env::args().collect();

    let mut prompt: String = "".to_string();
    if args.len() == 2
    // prompt as an arg
    {
        prompt = args[1].clone()
    } else
    // prompt as a stdin pipe
    {
        io::stdin()
            .read_to_string(&mut prompt)
            .expect("failed to read stdin");
    }

    match get_completion(&api_key, &prompt).await {
        Ok(response) => {
            println!("{}", response.completion);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
