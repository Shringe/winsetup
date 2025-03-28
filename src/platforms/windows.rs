use std::path::PathBuf;
use std::env;
use std::io::Cursor;
use reqwest::{blocking::Client, blocking::Response, Url};

struct DownloadManager {
    dir: PathBuf,
    client: Client,
}

impl DownloadManager {
    pub fn new() -> Self {
        let mut dir = env::temp_dir();
        dir.push("winsetup");

        let http_client_builder = Client::builder();
        let http_client = http_client_builder.build().unwrap();

        Self {
            dir: dir,
            client: http_client,
        }
    }

    pub fn get_file(&self, url: String, file_name: String) {
        let response = self.client.get(url).send();

        if response.is_err() {
                panic!("Error while retrieving data: {:#?}", response.err());
        } else {            
               let body = response.unwrap().bytes().unwrap();
               // Get the content of the response
               std::fs::write(file_name , &body).expect("Reference proteome download failed for {file_name}");
        }
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
        self.manager.get_file("https://github.com/jtroo/kanata/releases/download/v1.8.0/kanata.exe".to_string(), "kanata.exe.test".to_string());
    }
}

pub fn install() {
    let scoop = Scoop::new();
    scoop.download();
}
