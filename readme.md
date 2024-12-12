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

Here's an example of how to use the `ai_rs` library with the `GeminiClient` and `xyzClient`:

```rust
use ai_rs::{init_logging, GeminiClient};

fn main() {
    init_logging();

    let gemini_ai = GeminiClient::setup("your_api_key").model("gemini-1.5-pro");
    let gemini_response = gemini_ai.generate_content("Hello, Gemini!");
    println!("{}", gemini_response);
}
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
