# ai_rs

`ai_rs` is a Rust library that provides a unified interface to interact with multiple AI models. This library is designed to be modular, allowing you to easily add and use different AI models.

## Features

- Modular design: Each AI model is implemented in its own module.
- Easy setup: Initialize and configure AI models with simple method calls.
- Logging: Configurable logging levels using an `.env` file.

## Installation

Add `ai_rs` to your `Cargo.toml`:

```toml
[dependencies]
ai_rs = "0.0.1"
```

## Usage

### Example

Here's an example of how to use the `ai_rs` library with the `GeminiClient`:

```rust
use ai_rs::{init_logging, GeminiClient};

#[tokio::main]
async fn main() {
    init_logging();

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
```

### Gemini API Support

The library provides comprehensive support for Google's Gemini API with the following features:

#### Basic Usage

```rust
use ai_rs::{GeminiClient, GenerationConfig};

let client = GeminiClient::new("your_api_key", "gemini-1.5-pro");

// Simple text generation
let response = client.generate_content("Tell me a joke").await?;
if let Some(text) = response.get_text() {
    println!("{}", text);
}
```

#### Advanced Configuration

```rust
let config = GenerationConfig {
    temperature: Some(0.7),
    max_output_tokens: Some(100),
    top_p: Some(0.8),
    top_k: Some(40),
    candidate_count: None,
    stop_sequences: None,
};

let response = client.generate_content_with_config("Write a haiku", config).await?;
```

#### Streaming Support

```rust
use futures_util::StreamExt;

let mut stream = client.stream_content("Count from 1 to 5").await?;
while let Some(result) = stream.next().await {
    match result {
        Ok(chunk) => {
            if let Some(text) = chunk.get_text() {
                print!("{}", text);
            }
        }
        Err(e) => println!("Stream error: {}", e),
    }
}
```

#### Structured Requests

```rust
use ai_rs::{GenerateContentRequest, Content, Part};

let request = GenerateContentRequest {
    contents: vec![Content {
        role: "user".to_string(),
        parts: vec![Part {
            text: Some("Explain quantum computing".to_string()),
            inline_data: None,
        }],
    }],
    generation_config: Some(config),
    safety_settings: None,
    tools: None,
};

let response = client.generate_content_with_request(request).await?;
```

#### Environment Setup

Set your Gemini API key as an environment variable:

```bash
export GEMINI_API_KEY="your_api_key_here"
```

Or create a `.env` file:

```
GEMINI_API_KEY=your_api_key_here
RUST_LOG=info
```

### Logging

The library uses the `log` crate for logging and the `env_logger` crate to configure logging levels via an `.env` file. Create a `.env` file in the root of your project to specify the logging level:

```
RUST_LOG=info
```

### Adding New Models

To add a new AI model, create a new module in the `src` directory and implement the necessary methods. Update `src/lib.rs` to export the new module.

For example, to add a new model called `xyz`:

1. Create a folder for the `xyz` model:

```sh
mkdir src/xyz
```

2. Create a `xyz.rs` file inside the `xyz` folder with proper api calls and test cases. Sample code:

```rust
use log::{info, debug, error};

pub struct xyzClient {
  api_key: String,
  model: String,
}

impl xyzClient {
  pub fn setup(api_key: &str) -> Self {
      info!("Setting up xyzClient with API key");
      xyzClient {
          api_key: api_key.to_string(),
          model: String::new(),
      }
  }

  pub fn model(mut self, model: &str) -> Self {
      info!("Setting model to {}", model);
      self.model = model.to_string();
      self
  }

  pub fn generate_content(&self, prompt: &str) -> String {
      info!("Generating content for prompt: '{}'", prompt);
      // Mock implementation of content generation
      let response = format!(
          "Generated content for prompt: '{}', using model: '{}'",
          prompt, self.model
      );
      debug!("Generated response: {}", response);
      response
  }
}
```

3. Update `src/lib.rs` to export the `xyz` module:

   ```rust
   pub mod gemini;
   pub mod xyz;

   pub use gemini::GeminiClient;
   pub use xyz::xyzClient;

   use dotenv::dotenv;
   use std::env;

   pub fn init_logging() {
       dotenv().ok();
       env_logger::init();
   }
   ```

### testing

- Run example to ensure proper working:

```bash
cargo run --example file_name # for example main, ollama, gemini
```

- run build

```bash
cargo build --release
```

- run tests

```bash
cargo test
```

- get documentation

```bash
cargo doc --open --release
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

```

```

```

```
