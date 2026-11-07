use std::io::{self, Write};
use std::process::Command;
fn main(){
    println!("Enter a command !");
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();
        Commands(command);
    }
}

fn Commands(command: &str) {
    match command {
        "exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        "version" => {
            let output = Command::new("podman").arg("--version").output().expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        "start" => {
            let output = Command::new("podman").args(["start", "minecraft-test"]).output().expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        "stop" => {
            let output = Command::new("podman").args(["stop", "minecraft-test"]).output().expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}