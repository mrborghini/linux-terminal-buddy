use std::{collections::HashMap, env, fs::read_to_string};

pub struct DotEnvReader {
    file_name: String,
}

impl DotEnvReader {
    pub fn new<S: AsRef<str>>(file_name: S) -> Self {
        let file_name = file_name.as_ref().to_string();

        Self { file_name }
    }

    pub fn parse_and_set_env(&self) {
        // Read the file content
        let file_content = self.read_file_content();

        // Parse the file content into a HashMap of key-value pairs
        let env_vars = self.parse_lines(file_content);

        // Set environment variables
        self.set_env_vars(env_vars);
    }

    fn read_file_content(&self) -> String {
        match read_to_string(&self.file_name) {
            Ok(content) => content,
            Err(_) => "".to_string(),
        }
    }

    fn parse_lines(&self, content: String) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        for line in content.lines() {
            let split_line: Vec<&str> = line.split("#").collect();
            let trimmed_line = split_line[0].trim();

            // Skip empty lines and comments
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = self.parse_key_value(trimmed_line) {
                if value.is_empty() {
                    continue;
                }
                env_vars.insert(key, value);
            }
        }

        env_vars
    }

    fn parse_key_value(&self, line: &str) -> Option<(String, String)> {
        let mut split = line.splitn(2, '=');
        let key = split.next()?;
        let value = split.next().unwrap_or("").trim().to_string();
        Some((key.trim().to_string(), value))
    }

    fn set_env_vars(&self, env_vars: HashMap<String, String>) {
        for (key, value) in env_vars {
            // Check if the environment variable is already set
            if env::var(&key).is_ok() {
            } else {
                unsafe { env::set_var(&key, &value) };
            }
        }
    }
}
