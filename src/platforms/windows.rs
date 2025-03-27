use std::path::PathBuf;
use std::env;


struct DownloadManager {
    dir: PathBuf,
}

impl DownloadManager {
    pub fn new() -> Self {
        let mut dir = env::temp_dir();
        dir.push("winsetup");

        Self {
            dir: dir,
        }
    }

    pub fn get_file(&self, url: &str) {
    }
}


struct Scoop {
    manager: DownloadManager,
}

impl Scoop {
    pub fn new() -> Self {
        Self {
            manager: DownloadManager::new(),
        }
    }

    pub fn download(&self) {
    }
}

pub fn install() {
    let scoop = Scoop::new();
    scoop.download();
}
