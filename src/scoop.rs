use std::env;
use std::fs;
use std::process::Command;

pub fn scoop_cmd(args: &Vec<String>) -> String {
    let out = Command::new("scoop")
        .args(args)
        .output()
        .expect("Couldn't parse scoop");

    String::from_utf8_lossy(&out.stdout).to_string()
}

pub fn install_scoop() {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression"
        ])
        .status();
}
