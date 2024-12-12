use ai_rs::{init_logging, ollama::GenerateRequest, GeminiClient, OllamaClient};
use tokio::runtime::Runtime;

fn gemini() {
    let gemini_ai = GeminiClient::setup("your_api_key").model("gemini-1.5-pro");
    let gemini_response = gemini_ai.generate_content("Hello, Gemini!");
    println!("{}", gemini_response);
}

async fn ollama() {
    let ollama = OllamaClient::new("http://localhost:11434", "your_api_key");

    // Check if the service is active
    match ollama.active().await {
        Ok(active) => println!("Ollama service active: {}", active),
        Err(e) => eprintln!("Error checking Ollama service status: {}", e),
    }

    // Generate a completion
    // let generate_request = GenerateRequest {
    //     model: "your_model_name".to_string(),
    //     prompt: "Hello, Ollama!".to_string(),
    //     stream: None,
    //     options: None,
    // };

    // match ollama.generate_completion(generate_request).await {
    //     Ok(response) => println!("Generated response: {:?}", response),
    //     Err(e) => eprintln!("Error generating completion: {}", e),
    // }

    // // List models
    // match ollama.list_models().await {
    //     Ok(response) => println!("Available models: {:?}", response.models),
    //     Err(e) => eprintln!("Error listing models: {}", e),
    // }

    // // Show model info
    // match ollama.show_model_info("your_model_name").await {
    //     Ok(model_info) => println!("Model info: {:?}", model_info),
    //     Err(e) => eprintln!("Error showing model info: {}", e),
    // }
}

fn main() {
    init_logging();
    gemini();

    // Create a new Tokio runtime to run the async ollama function
    let rt = Runtime::new().unwrap();
    rt.block_on(ollama());
}
