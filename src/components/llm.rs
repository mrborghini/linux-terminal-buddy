use std::fs;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub timestamp: String,
}

pub struct LLM {
    conversation: Vec<Message>,
    llm_provider: Box<dyn LLMProvider>,
}

impl LLM {
    pub fn new(llm_provider: Box<dyn LLMProvider>) -> Self {
        Self {
            conversation: Vec::new(),
            llm_provider,
        }
    }

    fn add_message(&mut self, message: Message) {
        self.conversation.push(message);
    }

    fn get_system_message(&self, task: String) -> String {
        let content = fs::read_to_string("system_message.md")
            .unwrap_or("{{TASK}}".to_string())
            .replace("{{TASK}}", &task);
        content
    }

    pub async fn run(&mut self, task: String) {
        self.add_message(Message {
            content: self.get_system_message(task),
            role: "system".to_string(),
            timestamp: Local::now().to_rfc2822(),
        });
        println!(
            "{}",
            self.llm_provider.get_llm_message(&self.conversation).await
        );

        self.conversation = Vec::new();
    }
}

#[async_trait::async_trait]
pub trait LLMProvider {
    async fn get_llm_message(&self, message: &Vec<Message>) -> String;
}
