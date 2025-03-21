use components::{LLM, Ollama};

mod components;

#[tokio::main]
async fn main() {
    let ollama = Ollama::new("http://localhost:11434".to_string());
    let mut llm = LLM::new(Box::new(ollama));

    println!("What is my task?");
    let line = components::utils::read_line(Some("> "));
    println!("You said: {}", line);
    llm.run(line).await;
}
