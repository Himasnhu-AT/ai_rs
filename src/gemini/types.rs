use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request structure for generating content with Gemini
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateContentRequest {
    /// The contents to generate a response for
    pub contents: Vec<Content>,
    /// Generation configuration
    pub generation_config: Option<GenerationConfig>,
    /// Safety settings
    pub safety_settings: Option<Vec<SafetySetting>>,
    /// Tools to use
    pub tools: Option<Vec<Tool>>,
}

/// Content structure for Gemini API
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    /// The role of the content (user, model, etc.)
    pub role: String,
    /// The parts of the content
    pub parts: Vec<Part>,
}

/// Part of content (text, image, etc.)
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    /// The text content
    pub text: Option<String>,
    /// Inline data (for images, etc.)
    pub inline_data: Option<InlineData>,
}

/// Inline data for parts (images, etc.)
#[derive(Debug, Serialize, Deserialize)]
pub struct InlineData {
    /// MIME type of the data
    pub mime_type: String,
    /// The actual data
    pub data: String,
}

/// Generation configuration for Gemini
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Temperature for generation
    pub temperature: Option<f32>,
    /// Top-k sampling
    pub top_k: Option<i32>,
    /// Top-p sampling
    pub top_p: Option<f32>,
    /// Maximum output tokens
    pub max_output_tokens: Option<i32>,
    /// Candidate count
    pub candidate_count: Option<i32>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
}

/// Safety setting for content generation
#[derive(Debug, Serialize, Deserialize)]
pub struct SafetySetting {
    /// The category of safety setting
    pub category: String,
    /// The threshold for the safety setting
    pub threshold: String,
}

/// Tool definition for Gemini
#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    /// Function declarations
    pub function_declarations: Vec<FunctionDeclaration>,
}

/// Function declaration for tools
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    /// Name of the function
    pub name: String,
    /// Description of the function
    pub description: String,
    /// Parameters of the function
    pub parameters: serde_json::Value,
}

/// Response structure for generated content
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateContentResponse {
    /// The candidates generated
    pub candidates: Vec<Candidate>,
    /// Prompt feedback
    pub prompt_feedback: Option<PromptFeedback>,
    /// Usage metadata
    pub usage_metadata: Option<UsageMetadata>,
}

/// Candidate response from Gemini
#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    /// The content of the candidate
    pub content: Content,
    /// The finish reason
    pub finish_reason: Option<String>,
    /// The index of the candidate
    pub index: i32,
    /// Safety ratings
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

/// Safety rating for content
#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    /// The category of safety
    pub category: String,
    /// The probability of the safety rating
    pub probability: String,
}

/// Prompt feedback
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFeedback {
    /// Safety ratings for the prompt
    pub safety_ratings: Vec<SafetyRating>,
}

/// Usage metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    /// Prompt token count
    pub prompt_token_count: i32,
    /// Candidates token count
    pub candidates_token_count: i32,
    /// Total token count
    pub total_token_count: i32,
}

/// Stream response structure for Gemini
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamGenerateContentResponse {
    /// The candidates generated
    pub candidates: Vec<Candidate>,
    /// Prompt feedback
    pub prompt_feedback: Option<PromptFeedback>,
    /// Usage metadata
    pub usage_metadata: Option<UsageMetadata>,
}

impl GenerateContentResponse {
    /// Gets the text response from the first candidate
    pub fn get_text(&self) -> Option<String> {
        self.candidates.first().and_then(|candidate| {
            candidate
                .content
                .parts
                .first()
                .and_then(|part| part.text.clone())
        })
    }
}

impl StreamGenerateContentResponse {
    /// Gets the text response from the first candidate
    pub fn get_text(&self) -> Option<String> {
        self.candidates.first().and_then(|candidate| {
            candidate
                .content
                .parts
                .first()
                .and_then(|part| part.text.clone())
        })
    }
}
