use std::process::Command;

use crate::cli;

pub const GAME_PROGRAMS: [&str; 2] = ["games/prismlauncher", "games/steam"];
pub const GAME_BUCKETS: [&str; 1] = ["games"];

pub const ACADEMIC_PROGRAMS: [&str; 3] = [
    "extras/zen-browser",
    "extras/thunderbird",
    "extras/libreoffice",
];
pub const ACADEMIC_BUCKETS: [&str; 1] = ["extras"];

pub const PROGRAMMING_PROGRAMS: [&str; 3] = ["main/neovim", "main/python", "extras/alacritty"];
pub const PROGRAMMING_BUCKETS: [&str; 2] = ["main", "extras"];

pub struct Scoop<'a> {
    pub cmd_args: &'a cli::Args,
}

impl Scoop<'_> {
    fn cmd(&self, args: &Vec<&str>) {
        if self.cmd_args.dryrun {
            println!("> scoop {}", args.join(" "));
        } else {
            let _ = Command::new("powershell")
                .arg("-Command")
                .arg("scoop")
                .args(args)
                .output()
                .expect("Couldn't parse scoop");
        }
    }

    pub fn uninstall(&self) {
        self.cmd(&vec!["uninstall", "scoop"]);
    }

    pub fn install(&self) {
        if self.cmd_args.dryrun {
            println!("<Scoop installation command>");
        } else {
            let _ = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression"
                ])
                .status();
        }

        self.cmd(&vec!["install", "git"]);
    }

    pub fn update(&self) {
        self.cmd(&vec!["update"]);
    }

    pub fn add_buckets(&self, buckets: &Vec<&str>) {
        let mut cmd = vec!["buckets", "add"];
        cmd.extend(buckets);
        self.cmd(&cmd);
    }

    pub fn add_programs(&self, programs: &Vec<&str>) {
        let mut cmd = vec!["install"];
        cmd.extend(programs);
        self.cmd(&cmd);
    }
}
