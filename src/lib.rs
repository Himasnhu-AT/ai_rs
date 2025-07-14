pub mod gemini;
pub mod ollama;

pub use gemini::{
    Candidate, Content, GeminiClient, GenerateContentRequest, GenerateContentResponse,
    GenerationConfig, InlineData, Part, SafetyRating, SafetySetting, StreamGenerateContentResponse,
    Tool, UsageMetadata,
};
pub use ollama::OllamaClient;

use dotenv::dotenv;

pub fn init_logging() {
    dotenv().ok();
    env_logger::init();
}
