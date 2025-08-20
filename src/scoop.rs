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
    fn scoop_cmd(&self, args: &Vec<&str>) {
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

    pub fn install_scoop(&self) {
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
    }

    pub fn update_scoop(&self) {
        self.scoop_cmd(&vec!["update"]);
    }

    pub fn add_buckets(&self, buckets: &Vec<&str>) {
        // Git needed for buckets
        self.scoop_cmd(&vec!["install", "git"]);

        let mut buckets_cmd = vec!["buckets", "add"];
        buckets_cmd.extend(buckets);
        self.scoop_cmd(&buckets_cmd);
    }
}
