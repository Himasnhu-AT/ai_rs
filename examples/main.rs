use ai_rs::{init_logging, GeminiClient};

fn gemini() {
    let gemini_ai = GeminiClient::setup("your_api_key").model("gemini-1.5-pro");
    let gemini_response = gemini_ai.generate_content("Hello, Gemini!");
    println!("{}", gemini_response);
}

fn main() {
    init_logging();
    gemini();
}
