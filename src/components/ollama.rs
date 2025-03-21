use super::{LLMProvider, Message, get_reqwest_client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct OllamaBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    options: OllamaOptions,
    format: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OllamaOptions {
    num_ctx: i32,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: Message,
}

pub struct Ollama {
    base_url: String,
    model: String,
    num_ctx: i32,
    client: reqwest::Client,
}

#[async_trait::async_trait]
impl LLMProvider for Ollama {
    async fn get_llm_message(
        &self,
        messages: &Vec<Message>,
        format: Option<serde_json::Value>,
    ) -> Message {
        let url = format!("{}/api/chat", self.base_url);

        let body = serde_json::to_string(&OllamaBody {
            model: self.model.to_string(),
            messages: messages.to_vec(),
            stream: false,
            options: OllamaOptions {
                num_ctx: self.num_ctx,
            },
            format,
        })
        .unwrap();

        let response = self
            .client
            .post(url)
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_respose = serde_json::from_str::<OllamaResponse>(&response).unwrap();
        parsed_respose.message
    }
}

impl Ollama {
    pub fn new(ollama_url: String, model: String, num_ctx: i32) -> Self {
        Self {
            base_url: ollama_url,
            model,
            num_ctx,
            client: get_reqwest_client(),
        }
    }
}
