use crate::gemini::types::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
    StreamGenerateContentResponse,
};
use futures_util::{Stream, StreamExt};
use log::{debug, error, info, warn};
use reqwest::Client;
use serde::de::Error as SerdeError;
use serde_json::json;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

/// Custom error type to handle different error scenarios
#[derive(Debug)]
pub enum GeminiClientError {
    /// Error related to the request
    RequestError(String),
    /// Network-related error
    NetworkError(reqwest::Error),
    /// Error while parsing JSON
    ParseError(serde_json::Error),
    /// API error from Gemini
    ApiError(String),
}

impl fmt::Display for GeminiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeminiClientError::RequestError(msg) => write!(f, "Request error: {}", msg),
            GeminiClientError::NetworkError(err) => write!(f, "Network error: {}", err),
            GeminiClientError::ParseError(err) => write!(f, "Parse error: {}", err),
            GeminiClientError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for GeminiClientError {}

impl From<reqwest::Error> for GeminiClientError {
    fn from(err: reqwest::Error) -> Self {
        GeminiClientError::NetworkError(err)
    }
}

impl From<serde_json::Error> for GeminiClientError {
    fn from(err: serde_json::Error) -> Self {
        GeminiClientError::ParseError(err)
    }
}

/// Client for interacting with the Gemini API
#[derive(Debug)]
pub struct GeminiClient {
    api_key: String,
    model: String,
    base_url: String,
    client: Client,
}

impl GeminiClient {
    /// Creates a new instance of `GeminiClient`
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for Google AI Studio
    /// * `model` - The model to use (e.g., "gemini-1.5-pro")
    ///
    /// # Returns
    ///
    /// A new `GeminiClient` instance
    pub fn new(api_key: &str, model: &str) -> Self {
        info!("Creating new GeminiClient with model: {}", model);
        GeminiClient {
            api_key: api_key.to_string(),
            model: model.to_string(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            client: Client::new(),
        }
    }

    /// Legacy method for backward compatibility
    pub fn setup(api_key: &str) -> Self {
        Self::new(api_key, "gemini-1.5-pro")
    }

    /// Sets the model to use
    pub fn model(mut self, model: &str) -> Self {
        info!("Setting model to {}", model);
        self.model = model.to_string();
        self
    }

    /// Generates content based on a text prompt
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to generate content for
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GenerateContentResponse` or a `GeminiClientError`
    pub async fn generate_content(
        &self,
        prompt: &str,
    ) -> Result<GenerateContentResponse, GeminiClientError> {
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(prompt.to_string()),
                    inline_data: None,
                }],
            }],
            generation_config: None,
            safety_settings: None,
            tools: None,
        };

        self.generate_content_with_request(request).await
    }

    /// Generates content based on a structured request
    ///
    /// # Arguments
    ///
    /// * `request` - The `GenerateContentRequest` containing the content and configuration
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GenerateContentResponse` or a `GeminiClientError`
    pub async fn generate_content_with_request(
        &self,
        request: GenerateContentRequest,
    ) -> Result<GenerateContentResponse, GeminiClientError> {
        let url = format!("{}/models/{}:generateContent", self.base_url, self.model);
        info!("Generating content with URL: {}", url);
        debug!("GenerateContentRequest: {:?}", request);

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            debug!("Response JSON: {:?}", response_json);

            // Check for API errors in the response
            if let Some(error) = response_json.get("error") {
                let error_message = error.to_string();
                error!("Gemini API error: {}", error_message);
                return Err(GeminiClientError::ApiError(error_message));
            }

            let generate_response: GenerateContentResponse = serde_json::from_value(response_json)?;
            info!("Successfully generated content.");
            debug!("GenerateContentResponse: {:?}", generate_response);
            Ok(generate_response)
        } else {
            let error_message = response.text().await?;
            error!("Failed to generate content: {}", error_message);
            Err(GeminiClientError::RequestError(error_message))
        }
    }

    /// Streams content generation based on a text prompt
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to generate content for
    ///
    /// # Returns
    ///
    /// A `Result` containing a Stream of `StreamGenerateContentResponse` chunks or a `GeminiClientError`
    pub async fn stream_content(
        &self,
        prompt: &str,
    ) -> Result<
        impl Stream<Item = Result<StreamGenerateContentResponse, GeminiClientError>>,
        GeminiClientError,
    > {
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(prompt.to_string()),
                    inline_data: None,
                }],
            }],
            generation_config: None,
            safety_settings: None,
            tools: None,
        };

        self.stream_content_with_request(request).await
    }

    /// Streams content generation based on a structured request
    ///
    /// # Arguments
    ///
    /// * `request` - The `GenerateContentRequest` containing the content and configuration
    ///
    /// # Returns
    ///
    /// A `Result` containing a Stream of `StreamGenerateContentResponse` chunks or a `GeminiClientError`
    pub async fn stream_content_with_request(
        &self,
        request: GenerateContentRequest,
    ) -> Result<
        impl Stream<Item = Result<StreamGenerateContentResponse, GeminiClientError>>,
        GeminiClientError,
    > {
        let url = format!(
            "{}/models/{}:streamGenerateContent",
            self.base_url, self.model
        );
        info!("Streaming content with URL: {}", url);
        debug!("StreamRequest: {:?}", request);

        let response = self
            .client
            .post(&url)
            .header("x-goog-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let (tx, rx) = mpsc::channel(100);
            let stream = response.bytes_stream();

            tokio::spawn(async move {
                let mut stream = stream;
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(bytes) => {
                            let chunk_str = String::from_utf8_lossy(&bytes);
                            debug!("Received chunk: {}", chunk_str);

                            // Split by newlines and process each JSON object
                            for line in chunk_str.lines() {
                                if line.trim().is_empty() {
                                    continue;
                                }

                                // Remove "data: " prefix if present
                                let json_str = if line.starts_with("data: ") {
                                    &line[6..]
                                } else {
                                    line
                                };

                                if json_str.trim() == "[DONE]" {
                                    break;
                                }

                                match serde_json::from_str::<StreamGenerateContentResponse>(
                                    json_str,
                                ) {
                                    Ok(stream_response) => {
                                        if let Err(e) = tx.send(Ok(stream_response)).await {
                                            error!("Failed to send stream response: {}", e);
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to parse stream response: {}", e);
                                        if let Err(e) =
                                            tx.send(Err(GeminiClientError::ParseError(e))).await
                                        {
                                            error!("Failed to send error: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Stream error: {}", e);
                            if let Err(e) = tx.send(Err(GeminiClientError::NetworkError(e))).await {
                                error!("Failed to send network error: {}", e);
                            }
                            break;
                        }
                    }
                }
            });

            Ok(ReceiverStream::new(rx))
        } else {
            let error_message = response.text().await?;
            error!("Failed to start streaming: {}", error_message);
            Err(GeminiClientError::RequestError(error_message))
        }
    }

    /// Generates content with specific generation configuration
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to generate content for
    /// * `config` - The generation configuration
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GenerateContentResponse` or a `GeminiClientError`
    pub async fn generate_content_with_config(
        &self,
        prompt: &str,
        config: GenerationConfig,
    ) -> Result<GenerateContentResponse, GeminiClientError> {
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(prompt.to_string()),
                    inline_data: None,
                }],
            }],
            generation_config: Some(config),
            safety_settings: None,
            tools: None,
        };

        self.generate_content_with_request(request).await
    }

    /// Simple text generation method for backward compatibility
    pub fn generate_content_sync(&self, prompt: &str) -> String {
        // This is a blocking wrapper around the async method
        // Note: This is not ideal for production use, but maintains backward compatibility
        let rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(self.generate_content(prompt)) {
            Ok(response) => response
                .get_text()
                .unwrap_or_else(|| "No response generated".to_string()),
            Err(e) => format!("Error: {}", e),
        }
    }
}
