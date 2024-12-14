use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: Option<bool>,
    // pub options: Option<HashMap<String, serde_json::Value>>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: Option<String>, // Add this field
    pub context: Option<Vec<i32>>,
    pub total_duration: Option<u64>,
    pub load_duration: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u64>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u64>,
}

impl GenerateResponse {
    pub fn merge(&mut self, other: GenerateResponse) {
        self.response.push_str(&other.response);
        self.done = other.done;
        self.done_reason = other.done_reason.or(self.done_reason.clone());
        self.context = other.context.or(self.context.clone());
        self.total_duration = other.total_duration.or(self.total_duration);
        self.load_duration = other.load_duration.or(self.load_duration);
        self.prompt_eval_count = other.prompt_eval_count.or(self.prompt_eval_count);
        self.prompt_eval_duration = other.prompt_eval_duration.or(self.prompt_eval_duration);
        self.eval_count = other.eval_count.or(self.eval_count);
        self.eval_duration = other.eval_duration.or(self.eval_duration);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}
