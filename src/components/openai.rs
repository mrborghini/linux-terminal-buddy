use serde::{Deserialize, Serialize};

use super::{LLMProvider, Message, get_reqwest_client};

#[derive(Debug, Serialize)]
struct OpenAIBody {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_name: String,
    json_schema: JsonSchema,
}

#[derive(Debug, Serialize)]
struct JsonSchema {
    name: String,
    schema: serde_json::Value,
    strict: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choices>,
}

#[derive(Debug, Deserialize)]
struct Choices {
    message: Message,
}

pub struct OpenAI {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

#[async_trait::async_trait]
impl LLMProvider for OpenAI {
    async fn get_llm_message(
        &self,
        messages: &Vec<Message>,
        format: Option<serde_json::Value>,
    ) -> Message {
        let url = "https://api.openai.com/v1/chat/completions";
        let body = serde_json::to_string(&OpenAIBody {
            model: self.model.to_string(),
            messages: messages.to_vec(),
            response_format: ResponseFormat {
                type_name: "json_schema".to_string(),
                json_schema: JsonSchema {
                    name: "default".to_string(),
                    schema: format.as_ref().unwrap().clone(),
                    strict: true,
                },
            },
        })
        .unwrap();

        let response = self
            .client
            .post(url)
            .bearer_auth(self.api_key.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_response = serde_json::from_str::<OpenAIResponse>(&response).unwrap();
        parsed_response.choices[0].message.clone()
    }
}

impl OpenAI {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            client: get_reqwest_client(),
            model,
        }
    }
}
