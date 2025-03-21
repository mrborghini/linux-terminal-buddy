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

    fn get_system_message(&self) -> String {
        format!(
            "You are a self prompting LLM. Your goal is to do the following task:

        **{{TASK}}**.

        You have access to a Linux terminal to do it. You can run any command using the
        `command` field. If you think the task is completely completed set the
        `task_complete` field to `true`. If the task is still in progress set the
        `task_complete` field to `false`"
        )
    }

    pub async fn run(&mut self, task: String) {
        self.add_message(Message {
            content: task,
            role: "system".to_string(),
            timestamp: Local::now().to_rfc2822(),
        });
        println!("{}", self.llm_provider.get_llm_message(&self.conversation).await);

        self.conversation = Vec::new();
    }
}

#[async_trait::async_trait]
pub trait LLMProvider {
    async fn get_llm_message(&self, message: &Vec<Message>) -> String;
}
