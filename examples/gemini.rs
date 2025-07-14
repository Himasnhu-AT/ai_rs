use ai_rs::{init_logging, Content, GeminiClient, GenerateContentRequest, GenerationConfig, Part};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    init_logging();

    // Get API key from environment variable
    let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    // Create a new Gemini client
    let client = GeminiClient::new(&api_key, "gemini-1.5-pro");

    println!("=== Basic Content Generation ===");
    match client
        .generate_content("Hello! Can you tell me a short joke?")
        .await
    {
        Ok(response) => {
            if let Some(text) = response.get_text() {
                println!("Response: {}", text);
            } else {
                println!("No text in response");
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Content Generation with Configuration ===");
    let config = GenerationConfig {
        temperature: Some(0.7),
        max_output_tokens: Some(100),
        top_p: Some(0.8),
        top_k: Some(40),
        candidate_count: None,
        stop_sequences: None,
    };

    match client
        .generate_content_with_config("Write a haiku about programming", config)
        .await
    {
        Ok(response) => {
            if let Some(text) = response.get_text() {
                println!("Response: {}", text);
            } else {
                println!("No text in response");
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Structured Request ===");
    let request = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: Some("Explain quantum computing in simple terms".to_string()),
                inline_data: None,
            }],
        }],
        generation_config: Some(GenerationConfig {
            temperature: Some(0.3),
            max_output_tokens: Some(200),
            top_p: Some(0.9),
            top_k: Some(50),
            candidate_count: None,
            stop_sequences: None,
        }),
        safety_settings: None,
        tools: None,
    };

    match client.generate_content_with_request(request).await {
        Ok(response) => {
            if let Some(text) = response.get_text() {
                println!("Response: {}", text);
            } else {
                println!("No text in response");
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Streaming Content ===");
    match client.stream_content("Count from 1 to 5").await {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        if let Some(text) = chunk.get_text() {
                            print!("{}", text);
                        }
                    }
                    Err(e) => {
                        println!("\nStream error: {}", e);
                        break;
                    }
                }
            }
            println!(); // New line after streaming
        }
        Err(e) => println!("Error starting stream: {}", e),
    }

    println!("\n=== Backward Compatibility (Sync Method) ===");
    let sync_response = client.generate_content_sync("What is the capital of France?");
    println!("Sync Response: {}", sync_response);
}
