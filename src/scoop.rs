use std::process::Command;

use crate::cli;

const PROGRAMS: [&str; 7] = [
    "neovim",
    "python",
    "alacritty",
    "thunderbird",
    "zen-browser",
    "libreoffice",
    "prismlauncher",
];

const BUCKETS: [&str; 3] = ["main", "games", "extras"];

pub struct Scoop<'a> {
    pub cmd_args: &'a cli::Args,
}

impl Scoop<'_> {
    fn scoop_cmd(&self, args: &Vec<&str>) {
        if self.cmd_args.dryrun {
            println!("> scoop {}", args.join(" "));
        } else {
            let out = Command::new("powershell")
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

    pub fn add_buckets(&self) {
        for b in BUCKETS {
            let args = vec!["bucket", "add", b];
            self.scoop_cmd(&args);
        }
    }

    pub fn uninstall_scoop(&self) {
        let args = vec!["uninstall", "scoop"];
        self.scoop_cmd(&args);
    }

    pub fn install_programs(&self) {
        for p in PROGRAMS {
            let args = vec!["install", p];
            self.scoop_cmd(&args);
        }
    }
}
