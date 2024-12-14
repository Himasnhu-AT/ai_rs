use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

/// Request structure for generating a completion
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    /// The model to use for generation
    pub model: String,
    /// The prompt to generate a response for
    pub prompt: String,
    /// Whether to stream the response
    pub stream: Option<bool>,
    /// Additional options for the generation
    // pub options: Option<HashMap<String, serde_json::Value>>,
    pub options: Option<serde_json::Value>,
}

/// Response structure for a generated completion
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResponse {
    /// The model used for generation
    pub model: String,
    /// The creation timestamp of the response
    pub created_at: String,
    /// The generated response
    pub response: String,
    /// Whether the generation is done
    pub done: bool,
    /// The reason why the generation is done
    pub done_reason: Option<String>,
    /// The context of the response
    pub context: Option<Vec<i32>>,
    /// The total duration of the generation
    pub total_duration: Option<u64>,
    /// The duration to load the model
    pub load_duration: Option<u64>,
    /// The count of prompt evaluations
    pub prompt_eval_count: Option<u32>,
    /// The duration of prompt evaluations
    pub prompt_eval_duration: Option<u64>,
    /// The count of evaluations
    pub eval_count: Option<u32>,
    /// The duration of evaluations
    pub eval_duration: Option<u64>,
}

impl GenerateResponse {
    /// Merges another `GenerateResponse` into this one
    ///
    /// # Arguments
    ///
    /// * `other` - The other `GenerateResponse` to merge
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

/// Response structure for listing models
#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// The list of models
    pub models: Vec<ModelInfo>,
}

/// Information about a model
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    /// The name of the model
    pub name: String,
    /// The last modified timestamp of the model
    pub modified_at: String,
    /// The size of the model
    pub size: u64,
    /// The digest of the model
    pub digest: String,
    /// The details of the model
    pub details: ModelDetails,
}

/// Details about a model
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDetails {
    /// The format of the model
    pub format: String,
    /// The family of the model
    pub family: String,
    /// The families the model belongs to
    pub families: Option<Vec<String>>,
    /// The parameter size of the model
    pub parameter_size: String,
    /// The quantization level of the model
    pub quantization_level: String,
}
