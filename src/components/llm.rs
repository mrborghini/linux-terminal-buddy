use super::Shell;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;

use super::read_line;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub content: String,
    pub role: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct ParsedMessageContent {
    command: Option<String>,
    task_complete: bool,
    thinking_process: String,
}

pub struct LLM {
    conversation: Vec<Message>,
    llm_provider: Box<dyn LLMProvider>,
    shell: Shell,
    always_confirm: bool,
}

impl LLM {
    pub fn new(llm_provider: Box<dyn LLMProvider>, shell: Shell, always_confirm: bool) -> Self {
        Self {
            conversation: Vec::new(),
            llm_provider,
            shell,
            always_confirm,
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

    fn get_format(&self) -> Option<serde_json::Value> {
        let format = fs::read_to_string("llm_format.json")
            .ok()
            .map(|f| serde_json::from_str(&f).unwrap());
        format
    }

    fn ensure_allowed(&self) -> bool {
        if self.always_confirm {
            let allow_command = read_line(Some("Do you want to execute the command? (y/n) > "));
            return allow_command.to_lowercase() == "y";
        }
        true
    }

    pub async fn run(&mut self, task: String) {
        self.add_message(Message {
            content: self.get_system_message(task.to_owned()),
            role: "system".to_string(),
            timestamp: Some(Local::now().to_rfc2822()),
        });

        self.add_message(Message {
            content: task,
            role: "user".to_string(),
            timestamp: Some(Local::now().to_rfc2822()),
        });

        let mut no_command_count = 0;

        loop {
            let llm_message = self
                .llm_provider
                .get_llm_message(&self.conversation, self.get_format())
                .await;

            self.add_message(llm_message.clone());

            let parsed_message =
                serde_json::from_str::<ParsedMessageContent>(&llm_message.content).unwrap();

            println!("{}", parsed_message.thinking_process);

            if parsed_message.task_complete {
                break;
            }

            if no_command_count >= 3 {
                println!("The LLM is not able to understand the command.");
                self.add_message(Message {
                    content: format!("You didn't provide a command {} times. Please remember to add a command to the `command` field.", no_command_count),
                    role: "user".to_string(),
                    timestamp: Some(Local::now().to_rfc2822()),
                });
            }

            if parsed_message.command.is_none() {
                no_command_count += 1;
                continue;
            }

            no_command_count = 0;

            println!(
                "The LLM wants to execute: {}",
                parsed_message.command.clone().unwrap()
            );

            let cmd = &parsed_message.command.as_ref().unwrap().clone();

            if self.ensure_allowed() {
                let command_output = self.shell.execute_command(cmd);
                println!("{} executed: {}", cmd, command_output);
                self.add_message(Message {
                    content: command_output.clone(),
                    role: "user".to_string(),
                    timestamp: Some(Local::now().to_rfc2822()),
                });
                continue;
            }
            println!("`{}` not executed!", cmd);
            self.add_message(Message {
                content: format!("The command `{}` was not approved by the user.", cmd),
                role: "user".to_string(),
                timestamp: Some(Local::now().to_rfc2822()),
            });
        }

        self.conversation = Vec::new();
    }
}

#[async_trait::async_trait]
pub trait LLMProvider {
    async fn get_llm_message(
        &self,
        message: &Vec<Message>,
        format: Option<serde_json::Value>,
    ) -> Message;
}
