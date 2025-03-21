
mod components;

#[tokio::main]
async fn main() {
    println!("What is my task?");
    let line = components::utils::read_line(Some("> "));
    println!("You said: {}", line);
}
