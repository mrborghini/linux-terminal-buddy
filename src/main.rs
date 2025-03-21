use std::env;

use components::{DotEnvReader, LLM, LLMProvider, Ollama, OpenAI, Shell, read_line};
mod components;

fn select_llm() -> Box<dyn LLMProvider> {
    let openai_key = env::var("OPENAI_KEY").unwrap_or("".to_string());
    let openai_model = env::var("OPENAI_MODEL").unwrap_or("gpt-4o-mini".to_string());
    if !openai_key.is_empty() {
        println!("Using OpenAI: {}", openai_model);
        let llm = OpenAI::new(openai_key, openai_model);
        return Box::new(llm);
    }

    // Default to Ollama
    let ollama_url = env::var("OLLAMA_BASE_URL").unwrap_or("http://localhost:11434".to_string());
    let ollama_model = env::var("OLLAMA_MODEL").unwrap_or("llama3.2".to_string());
    let ollama_num_ctx = env::var("OLLAMA_NUM_CTX")
        .unwrap_or("2048".to_string())
        .parse()
        .unwrap_or(2048);
    let llm = Ollama::new(ollama_url, ollama_model.clone(), ollama_num_ctx);

    println!("Using Ollama: {}", ollama_model);
    return Box::new(llm);
}

#[tokio::main]
async fn main() {
    let dotenv_reader = DotEnvReader::new(".env");
    dotenv_reader.parse_and_set_env();
    let shell = Shell::new("sh".to_string());

    let allow_all_commands = env::var("ALLOW_ALL_COMMANDS")
        .unwrap_or("false".to_string())
        .to_lowercase()
        == "true";

    let mut llm = LLM::new(select_llm(), shell, !allow_all_commands);

    loop {
        println!("What is my task?");
        let line = read_line(Some("> "));
        println!("You said: {}", line);
        llm.run(line).await;
    }
}
