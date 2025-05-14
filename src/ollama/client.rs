use crate::ollama::types::{GenerateRequest, GenerateResponse, ListModelsResponse};
use log::{debug, error, info, warn};
use reqwest::Client;
use serde::de::Error as SerdeError;
use serde_json::{json, Value};
use std::fmt;
use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use std::sync::Arc;

/// Custom error type to handle different error scenarios
#[derive(Debug)]
pub enum OllamaClientError {
    /// Error related to the request
    RequestError(String),
    /// Network-related error
    NetworkError(reqwest::Error),
    /// Error while parsing JSON
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

/// Client for interacting with the Ollama API
#[derive(Debug)]
pub struct OllamaClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl OllamaClient {
    /// Creates a new instance of `OllamaClient`
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Ollama API
    /// * `api_key` - The API key for authentication
    ///
    /// # Returns
    ///
    /// A new `OllamaClient` instance
    pub fn new(base_url: &str, api_key: &str) -> Self {
        info!("Creating new OllamaClient with base_url: {}", base_url);
        OllamaClient {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// Checks if the Ollama service is active
    ///
    /// # Returns
    ///
    /// A `Result` containing `true` if the service is active, or an `OllamaClientError` otherwise
    pub async fn active(&self) -> Result<bool, OllamaClientError> {
        let url = format!("{}", self.base_url);
        info!("Checking if the service is active at URL: {}", url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            info!("Service is active.");
            Ok(true)
        } else {
            warn!("Service is not active. Status: {}", response.status());
            Ok(false)
        }
    }

    /// Generates a completion based on the provided request
    ///
    /// # Arguments
    ///
    /// * `request` - The `GenerateRequest` containing the model and prompt
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GenerateResponse` or an `OllamaClientError`
    pub async fn generate_completion(
        &self,
        request: GenerateRequest,
    ) -> Result<GenerateResponse, OllamaClientError> {
        let url = format!("{}/api/generate", self.base_url);
        info!("Generating completion with URL: {}", url);
        debug!("GenerateRequest: {:?}", request);

        // Build the JSON request body conditionally
        let mut json_body = json!({
            "model": request.model,
            "prompt": request.prompt,
        });

        if let Some(stream) = request.stream {
            json_body["stream"] = json!(stream);
        }

        if let Some(options) = request.options {
            json_body["options"] = options;
        }

        debug!("Sending body: {:?}", json_body.to_string());

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json_body)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            debug!("text response received: {:?}", response_text);

            // Split the response text by newlines and parse each JSON object
            let mut final_response: Option<GenerateResponse> = None;
            for line in response_text.lines() {
                let generate_response: GenerateResponse = serde_json::from_str(line)?;
                if let Some(ref mut existing_response) = final_response {
                    existing_response.merge(generate_response);
                } else {
                    final_response = Some(generate_response);
                }
            }

            if let Some(generate_response) = final_response {
                info!("Successfully generated completion.");
                debug!("GenerateResponse: {:?}", generate_response);
                Ok(generate_response)
            } else {
                Err(OllamaClientError::ParseError(SerdeError::custom(
                    "No valid JSON objects found in response",
                )))
            }
        } else {
            let error_message = response.text().await?;
            error!("Failed to generate completion: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }

    /// Streams a completion response chunk by chunk based on the provided request
    ///
    /// # Arguments
    ///
    /// * `request` - The `GenerateRequest` containing the model and prompt
    ///
    /// # Returns
    ///
    /// A `Result` containing a Stream of `GenerateResponse` chunks or an `OllamaClientError`
    pub async fn stream_completion(
        &self,
        mut request: GenerateRequest,
    ) -> Result<impl Stream<Item = Result<GenerateResponse, OllamaClientError>>, OllamaClientError> {
        // Force streaming to be enabled
        request.stream = Some(true);
        
        let url = format!("{}/api/generate", self.base_url);
        info!("Streaming completion with URL: {}", url);
        debug!("StreamRequest: {:?}", request);
        
        // Build the JSON request body conditionally
        let mut json_body = json!({
            "model": request.model,
            "prompt": request.prompt,
            "stream": true,
        });
        
        if let Some(options) = request.options {
            json_body["options"] = options;
        }
        
        debug!("Sending body: {:?}", json_body.to_string());
        
        let auth_header = format!("Bearer {}", self.api_key);
        let client = self.client.clone();
        
        // Create a response stream
        let response = client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&json_body)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let error_message = response.text().await?;
            error!("Failed to stream completion: {}", error_message);
            return Err(OllamaClientError::RequestError(error_message));
        }
        
        // Create a channel for passing chunks
        let (tx, rx) = mpsc::channel(32);
        let tx = Arc::new(tx);
        
        // Create a stream from the response
        let stream = response.bytes_stream();
        
        // Spawn a task to process the stream
        tokio::spawn(async move {
            let mut stream = stream;
            
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        // Process each line in the chunk
                        if let Ok(chunk_str) = String::from_utf8(chunk.to_vec()) {
                            for line in chunk_str.lines() {
                                if line.is_empty() {
                                    continue;
                                }
                                
                                match serde_json::from_str::<GenerateResponse>(line) {
                                    Ok(response) => {
                                        let tx = Arc::clone(&tx);
                                        if tx.send(Ok(response)).await.is_err() {
                                            // Receiver dropped, exit the loop
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        let tx = Arc::clone(&tx);
                                        if tx.send(Err(OllamaClientError::ParseError(e))).await.is_err() {
                                            // Receiver dropped, exit the loop
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let tx = Arc::clone(&tx);
                        let _ = tx.send(Err(OllamaClientError::NetworkError(e))).await;
                        break;
                    }
                }
            }
        });
        
        // Return the receiver as a stream
        Ok(ReceiverStream::new(rx))
    }

    /// Lists available models
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ListModelsResponse` or an `OllamaClientError`
    pub async fn list_models(&self) -> Result<ListModelsResponse, OllamaClientError> {
        let url = format!("{}/api/tags", self.base_url);
        info!("Listing models with URL: {}", url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let list_models_response: ListModelsResponse = response.json().await?;
            info!("Successfully listed models.");
            debug!("ListModelsResponse: {:?}", list_models_response);
            Ok(list_models_response)
        } else {
            let error_message = response.text().await?;
            error!("Failed to list models: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }

    /// Shows information about a specific model
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model
    ///
    /// # Returns
    ///
    /// A `Result` containing the model information as `serde_json::Value` or an `OllamaClientError`
    ///
    /// Use .response to get output sent by llm
    pub async fn show_model_info(&self, model: &str) -> Result<Value, OllamaClientError> {
        let url = format!("{}/api/show", self.base_url);
        info!("Showing model info for model: {} with URL: {}", model, url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "model": model }))
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            debug!("Received Response for show_model_info: {}", response_text);
            let model_info: Value = serde_json::from_str(&response_text)?;
            info!("Successfully retrieved model info.");
            debug!("ModelInfo: {:?}", model_info);
            Ok(model_info)
        } else {
            let error_message = response.text().await?;
            error!("Failed to show model info: {}", error_message);
            Err(OllamaClientError::RequestError(error_message))
        }
    }
}
