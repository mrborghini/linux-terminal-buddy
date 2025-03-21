use super::{LLMProvider, Message};

pub struct Ollama {
    base_url: String,
}

#[async_trait::async_trait]
impl LLMProvider for Ollama {
    async fn get_llm_message(&self, messages: &Vec<Message>) -> String {
        format!("Ollama ({}) says: {}", self.base_url, messages[0].content)
    }
}

impl Ollama {
    pub fn new(ollama_url: String) -> Self {
        Self {
            base_url: ollama_url,
        }
    }
}
