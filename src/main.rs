
use std::env;
use std::process::Command;

fn main() {
    
    let path = env::current_dir()
        .expect("Error: Could not get current executable path");

    let mut child = Command::new("cmd.exe")
        .args(["/C", &format!(r"{}\{}",path.to_str().unwrap(), "ChaTOR.exe")])
        .spawn()
        .expect("");

    child
        .wait()
        .expect("Error: Child process encountered an error");

}
