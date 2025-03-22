use dirs;
use std::env;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::str;

pub struct Shell {
    shell: String,
    history: Vec<String>,
    current_directory: String,
}

impl Shell {
    // Create a new Shell instance
    pub fn new(shell: String) -> Self {
        Shell {
            shell,
            history: Vec::new(),
            current_directory: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }

    // Execute a command in the shell
    pub fn execute_command(&mut self, command: &str) -> String {
        // Store the command in history
        self.history.push(command.to_string());

        // Handle 'cd' command
        if command.starts_with("cd ") {
            let (path, rest) = self.get_dir_and_rest(command.to_string());
            if let Err(e) = self.change_directory(path) {
                return format!("Error changing directory: {}", e);
            }

            if !rest.is_empty() {
                return self.execute_command(&rest);
            }

            return String::new(); // 'cd' doesn't return output
        }

        // Spawn the command in the current directory
        let mut child = Command::new(&self.shell)
            .arg("-c")
            .arg(command)
            .current_dir(&self.current_directory)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let stdout = child.stdout.take().expect("failed to capture stdout");
        let stderr = child.stderr.take().expect("failed to capture stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // Read and print stdout and stderr in real time
        let mut output = String::new();
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                output.push_str(&line);
                output.push('\n');
            }
        }

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
                output.push_str(&line);
                output.push('\n');
            }
        }

        // Wait for the child process to finish
        child.wait().expect("failed to wait on child");

        output.trim().to_string()
    }

    fn get_dir_and_rest(&self, command: String) -> (String, String) {
        let parts: Vec<&str> = command.splitn(2, "cd ").collect();
        if parts.len() < 2 {
            return (String::new(), command);
        }
        let path_and_rest = parts[1];
        let mut split = path_and_rest.splitn(2, " &&");
        let path = split.next().unwrap_or("").trim().to_string();
        let rest = split.next().unwrap_or("").trim().to_string();
        (path, rest)
    }

    // Handle the output from the command
    // fn handle_output(&self, output: Output) -> String {
    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     let stderr = String::from_utf8_lossy(&output.stderr);
    //     format!("{}{}", stdout, stderr).trim().to_string()
    // }

    // Change the current directory
    fn change_directory(&mut self, path: String) -> Result<(), std::io::Error> {
        let new_path = if path == "~" {
            dirs::home_dir().unwrap()
        } else {
            std::path::Path::new(&path).to_path_buf()
        };

        // Change directory and update the current directory
        env::set_current_dir(&new_path)?;
        self.current_directory = env::current_dir().unwrap().to_str().unwrap().to_string();
        Ok(())
    }
}
