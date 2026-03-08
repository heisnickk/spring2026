use std::io::{self, Write};
use std::process::Command;

// 1. Define the FileOperation Enum
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

// 3. Function to perform the operation
fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            let status = Command::new("ls")
                .arg(&dir)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                eprintln!("Error listing directory '{}'", dir);
            }
        }
        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(&file)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                eprintln!("Error displaying file '{}'", file);
            }
        }
        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);
            let status = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File '{}' created successfully.", file);
            } else {
                eprintln!("Failed to create file '{}'.", file);
            }
        }
        FileOperation::Remove(file) => {
            let status = Command::new("rm")
                .arg(&file)
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File '{}' removed successfully.", file);
            } else {
                eprintln!("Failed to remove file '{}'.", file);
            }
        }
        FileOperation::Pwd => {
            let output = Command::new("pwd")
                .output()
                .expect("Failed to execute pwd");

            if output.status.success() {
                let cwd = String::from_utf8_lossy(&output.stdout);
                println!("Current working directory: {}", cwd.trim());
            } else {
                eprintln!("Failed to get current directory.");
            }
        }
    }
}

// Helper function to get user input
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        // 2. Display menu
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = read_input("Enter your choice (0-5): ");

        match choice.as_str() {
            "1" => {
                let dir = read_input("Enter directory path: ");
                perform_operation(FileOperation::List(dir));
            }
            "2" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Display(file));
            }
            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(file, content));
            }
            "4" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(file));
            }
            "5" => {
                perform_operation(FileOperation::Pwd);
            }
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}