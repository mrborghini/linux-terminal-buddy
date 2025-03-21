use components::{LLM, Ollama, Shell, read_line};
mod components;

#[tokio::main]
async fn main() {
    let shell = Shell::new("zsh".to_string());
    let ollama = Ollama::new(
        "http://localhost:11434".to_string(),
        "llama3.1".to_string(),
        32768,
    );

    let mut llm = LLM::new(Box::new(ollama), shell, true);

    loop {
        println!("What is my task?");
        let line = read_line(Some("> "));
        println!("You said: {}", line);
        llm.run(line).await;
    }
}
