use dirs;
use include_dir::{Dir, DirEntry, File, include_dir};
use std::{fs, io::Result, path::PathBuf};

static DEFAULT_CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/config");

/// Initializes a default config file
fn init_file(file: &File<'_>, to: &PathBuf) -> Result<()> {
    let file_name = file.path().file_name().unwrap();
    let file_path = to.join(file_name);

    fs::write(&file_path, file.contents())?;
    Ok(())
}

/// Recursively initializes the default config
fn init_recursive(from: &Dir<'_>, to: &PathBuf) -> Result<()> {
    for entry in from.entries() {
        match entry {
            DirEntry::Dir(dir) => {
                let to = to.join(dir.path().file_name().unwrap());
                fs::create_dir(&to)?;
                init_recursive(dir, &to)?;
            }
            DirEntry::File(file) => init_file(&file, &to)?,
        }
    }

    Ok(())
}

fn get_startup_folder() -> Option<PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push("Microsoft");
        path.push("Windows");
        path.push("Start Menu");
        path.push("Programs");
        path.push("Startup");
        path
    })
}

pub struct Config {
    dryrun: bool,
    winsetup_path: PathBuf,
    autostart_vbs_path: PathBuf,
}

impl Config {
    pub fn new(dryrun: bool) -> Self {
        let winsetup_path = dirs::home_dir()
            .expect("Could not find home directory")
            .join("winsetup");
        let autostart_folder = get_startup_folder().unwrap();
        let autostart_vbs_path = autostart_folder.join("winsetup.vbs");

        Self {
            dryrun,
            winsetup_path,
            autostart_vbs_path,
        }
    }

    /// Instantializes configuration
    pub fn install(&self) {
        if self.winsetup_path.exists() {
            if self.dryrun {
                println!("<Removing {}>", self.winsetup_path.display());
            } else {
                fs::remove_dir_all(&self.winsetup_path)
                    .expect("Failed to remove existing winsetup directory");
            }
        }

        if self.dryrun {
            println!(
                "<Creating and instantializing {}>",
                self.winsetup_path.display()
            );
        } else {
            fs::create_dir(&self.winsetup_path).expect("Couldn't create winsetup directory");
            init_recursive(&DEFAULT_CONFIG, &self.winsetup_path)
                .expect("Couldn't instantialize config");
        }

        let autostart_vbs_content = format!(
            "CreateObject('Wscript.Shell').Run '{}', 0, True",
            self.winsetup_path.join("autostart.bat").display()
        );

        if self.dryrun {
            println!(
                "<Creating autostart.vbs script at {} with contents:\n{}\n>",
                self.autostart_vbs_path.display(),
                autostart_vbs_content
            );
        } else {
            let _ = fs::write(&self.autostart_vbs_path, &autostart_vbs_content);
        }
    }
}
