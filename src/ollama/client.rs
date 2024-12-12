use crate::ollama::types::{GenerateRequest, GenerateResponse, ListModelsResponse, ModelInfo};
use log::error;
use reqwest::Client;
use std::fmt;

// Custom error type to handle different error scenarios
#[derive(Debug)]
pub enum OllamaClientError {
    RequestError(String),
    NetworkError(reqwest::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for OllamaClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OllamaClientError::RequestError(msg) => write!(f, "Request error: {}", msg),
            OllamaClientError::NetworkError(err) => write!(f, "Network error: {}", err),
            OllamaClientError::ParseError(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for OllamaClientError {}

impl From<reqwest::Error> for OllamaClientError {
    fn from(err: reqwest::Error) -> Self {
        OllamaClientError::NetworkError(err)
    }
}

impl From<serde_json::Error> for OllamaClientError {
    fn from(err: serde_json::Error) -> Self {
        OllamaClientError::ParseError(err)
    }
}

#[derive(Debug)]
pub struct OllamaClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl OllamaClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        OllamaClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    pub async fn active(&self) -> Result<bool, OllamaClientError> {
        let url = format!("{}", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn generate_completion(
        &self,
        request: GenerateRequest,
    ) -> Result<GenerateResponse, OllamaClientError> {
        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let generate_response: GenerateResponse = response.json().await?;
            Ok(generate_response)
        } else {
            let error_message = response.text().await?;
            error!("Failed to generate completion: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }

    pub async fn list_models(&self) -> Result<ListModelsResponse, OllamaClientError> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let list_models_response: ListModelsResponse = response.json().await?;
            Ok(list_models_response)
        } else {
            let error_message = response.text().await?;
            error!("Failed to list models: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }

    pub async fn show_model_info(&self, model: &str) -> Result<ModelInfo, OllamaClientError> {
        let url = format!("{}/api/show", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "model": model }))
            .send()
            .await?;

        if response.status().is_success() {
            let model_info: ModelInfo = response.json().await?;
            Ok(model_info)
        } else {
            let error_message = response.text().await?;
            error!("Failed to show model info: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }
}
