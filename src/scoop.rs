use colored::Colorize;
use std::process::Command;

pub const GAME_PROGRAMS: [&str; 3] = ["games/prismlauncher", "games/steam", "extras/discord"];
pub const GAME_BUCKETS: [&str; 2] = ["games", "extras"];

pub const ACADEMIC_PROGRAMS: [&str; 3] = [
    "extras/zen-browser",
    "extras/thunderbird",
    "extras/libreoffice",
];
pub const ACADEMIC_BUCKETS: [&str; 1] = ["extras"];

pub const PROGRAMMING_PROGRAMS: [&str; 3] = ["main/neovim", "main/python", "extras/alacritty"];
pub const PROGRAMMING_BUCKETS: [&str; 2] = ["main", "extras"];

pub struct Scoop {
    pub dryrun: bool,
}

impl Scoop {
    /// Just executes the given scoop cmd, without printing any info
    /// Largely a helper method for other cmd methods
    fn execute_cmd(&self, args: &Vec<&str>) {
        if !self.dryrun {
            let _ = Command::new("powershell")
                .arg("-Command")
                .arg("scoop")
                .args(args)
                .output();
        }
    }

    fn cmd_ok(&self, args: &Vec<&str>) {
        let text = format!("> scoop {}", args.join(" "));
        print!("{}", text.magenta());
        print!("...");
        self.execute_cmd(args);
        println!("{}", "OK".green());
    }

    pub fn uninstall(&self) {
        self.cmd_ok(&vec!["uninstall", "scoop"]);
    }

    pub fn install(&self) {
        if self.dryrun {
            println!("<Scoop installation command>");
        } else {
            let _ = Command::new("powershell")
                .args([
                    // "-NoProfile",
                    "-Command",
                    "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression"
                ])
                .output();
        }
    }

    pub fn update(&self) {
        self.cmd_ok(&vec!["update"]);
    }

    pub fn add_buckets(&self, buckets: &Vec<&str>) {
        self.cmd_ok(&vec!["install", "git"]);
        for b in buckets {
            let cmd = vec!["bucket", "add", b];
            self.cmd_ok(&cmd);
        }
    }

    pub fn add_programs(&self, programs: &Vec<&str>) {
        for p in programs {
            let cmd = vec!["install", p];
            self.cmd_ok(&cmd);
        }
    }
}
