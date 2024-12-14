use ai_rs::{init_logging, ollama::GenerateRequest, OllamaClient};
use tokio::runtime::Runtime;

const URL: &str = "http://localhost:11434";
const MODEL: &str = "llama3.2:1b";
const API_KEY: &str = "none";

async fn ollama() {
    let ollama = OllamaClient::new(URL, API_KEY);

    // Check if the service is active
    match ollama.active().await {
        Ok(active) => println!("Ollama service active: {}", active),
        Err(e) => eprintln!("Error checking Ollama service status: {}", e),
    }

    // List models
    match ollama.list_models().await {
        Ok(response) => println!("Available models: {:?}", response.models.len()),
        Err(e) => eprintln!("Error listing models: {}", e),
    }

    // Show model info for llama3.2:1b
    match ollama.show_model_info(MODEL).await {
        #[allow(unused_variables)]
        Ok(model_info) => println!("Recieved Model info"),
        Err(e) => eprintln!("Error showing model info: {}", e),
    }

    // Generate a completion
    let generate_request = GenerateRequest {
        model: MODEL.to_string(),
        prompt: "Hello, llama!".to_string(),
        stream: None,
        options: None,
    };

    match ollama.generate_completion(generate_request).await {
        Ok(response) => println!("Generated response: {:?}", response.response),
        Err(e) => eprintln!("Error generating completion: {}", e),
    }
}

fn main() {
    // uncomment to not get logs
    init_logging();

    // Create a new Tokio runtime to run the async ollama function
    let rt = Runtime::new().unwrap();
    rt.block_on(ollama());
}
