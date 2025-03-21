use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub timestamp: String,
}

pub struct Llm {
    conversation: Vec<Message>,
}

impl Llm {
    pub fn new() -> Self {
        Self { conversation: Vec::new() }
    }
}

pub trait LLM {
    fn get_llm_message(&self, message: String) -> String;
}