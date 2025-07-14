use ai_rs::{init_logging, GeminiClient};

async fn gemini() {
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let gemini_ai = GeminiClient::new(&api_key, "gemini-1.5-pro");

    match gemini_ai.generate_content("Hello, Gemini!").await {
        Ok(response) => {
            if let Some(text) = response.get_text() {
                println!("{}", text);
            } else {
                println!("No response generated");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    init_logging();
    tokio::runtime::Runtime::new().unwrap().block_on(gemini());
}
