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

/// Instantializes configuration
/// TODO, link winsetup_path/autostart.bat to the autostart directory
pub fn install(dryrun: bool) {
    let winsetup_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join("winsetup");

    if winsetup_path.exists() {
        if dryrun {
            println!("<Removing {}>", winsetup_path.display());
        } else {
            fs::remove_dir_all(&winsetup_path)
                .expect("Failed to remove existing winsetup directory");
        }
    }

    if dryrun {
        println!("<Creating and instantializing {}>", winsetup_path.display());
    } else {
        fs::create_dir(&winsetup_path).expect("Couldn't create winsetup directory");
        init_recursive(&DEFAULT_CONFIG, &winsetup_path).expect("Couldn't instantialize config");
    }
}
