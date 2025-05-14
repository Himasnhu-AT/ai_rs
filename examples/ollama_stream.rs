use ai_rs::ollama::client::OllamaClient;
use ai_rs::ollama::types::GenerateRequest;
use futures_util::StreamExt;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Create an Ollama client
    // Note: Update these values with your actual Ollama server URL and API key
    let client = OllamaClient::new("http://localhost:11434", "");
    
    // Check if the Ollama service is active
    let is_active = client.active().await?;
    println!("Ollama service active: {}", is_active);
    
    if !is_active {
        return Err("Ollama service is not active".into());
    }
    
    // Create a request for streaming completion
    let request = GenerateRequest {
        model: "llama3.2:1b".to_string(), // Use an available model on your Ollama instance
        prompt: "Write a short poem about programming".to_string(),
        stream: Some(true),
        options: None,
    };
    
    println!("\nStreaming response for: {}\n", request.prompt);
    
    // Stream the completion
    let mut stream = client.stream_completion(request).await?;
    
    // Process each chunk as it arrives
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // Print just the response text from each chunk
                print!("{}", chunk.response);
                io::stdout().flush()?;
                
                // If this is the final chunk, print a newline
                if chunk.done {
                    println!("\n\nGeneration complete. Reason: {:?}", chunk.done_reason);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error while streaming: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}