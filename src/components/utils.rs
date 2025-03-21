pub fn read_line(message: Option<&str>) -> String {
    let mut buffer = String::new();
    print!("{}", message.unwrap_or(""));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn execute_command(command: String) -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    format!("{}{}", stdout, stderr).trim().to_string()
}
