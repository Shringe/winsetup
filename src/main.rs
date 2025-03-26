pub mod platforms;
pub mod cli;

use clap::Parser;
use platforms::platform::Platform;
use platforms::platform::System;

fn main() {
    let args = cli::Cli::parse();

    if args.debug {
        println!("{:#?}", args);
    }

    let platform;
    match std::env::consts::OS {
        "windows" => platform = System::Windows,
        _ => todo!(),
    }

    let platform = Platform {
        system: platform,
    };

    if args.install {
        platform.install();
    }
}
