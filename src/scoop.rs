use std::process::Command;

const PROGRAMS: [&str; 6] = [
    "neovim",
    "thunderbird",
    "zen-browser",
    "prismlauncher",
    "alacritty",
    "python",
];

const BUCKETS: [&str; 3] = ["main", "games", "extras"];

fn scoop_cmd(args: &Vec<&str>) -> String {
    let out = Command::new("powershell")
        // .arg("-NoProfile")
        .arg("-Command")
        .arg("scoop")
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

pub fn add_buckets() {
    for b in BUCKETS {
        let args = vec!["bucket", "add", b];
        scoop_cmd(&args);
    }
}

pub fn uninstall_scoop() {
    let args = vec!["uninstall", "scoop"];
    scoop_cmd(&args);
}

pub fn install_programs() {
    for p in PROGRAMS {
        let args = vec!["install", p];
        scoop_cmd(&args);
    }
}
