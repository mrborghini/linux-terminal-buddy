use super::LLM;

pub struct Ollama {}

impl LLM for Ollama {
    fn get_llm_message(&self, message: String) -> String {
        format!("Ollama says: {}", message)
    }
}
