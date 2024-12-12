pub mod gemini;
pub mod ollama;

pub use gemini::GeminiClient;
pub use ollama::OllamaClient;

use dotenv::dotenv;

pub fn init_logging() {
    dotenv().ok();
    env_logger::init();
}
