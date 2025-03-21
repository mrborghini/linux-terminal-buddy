pub fn read_line(message: Option<&str>) -> String {
    let mut buffer = String::new();
    print!("{}", message.unwrap_or(""));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn get_reqwest_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .unwrap()
}
