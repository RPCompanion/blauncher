
use std::env;
use std::process::Command;

fn main() {
    
    let path = env::current_dir()
        .expect("Error: Could not get current executable path");

    let output = Command::new("cmd.exe")
        .args(["/C", &format!(r"{}\{}",path.to_str().unwrap(), "ChaTOR.exe")])
        .output()
        .expect("Error: Could not execute command");

    std::fs::write("log.txt", &output.stdout)
        .expect("Error: Could not write to file");

}
