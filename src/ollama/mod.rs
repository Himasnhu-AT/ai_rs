pub mod client;
pub mod types;
// pub mod utils;

pub use client::OllamaClient;
pub use types::{GenerateRequest, GenerateResponse, ListModelsResponse, ModelInfo};
