use clap::Parser;
use std::{thread, time};

mod cli;
mod scoop;

fn main() {
    let args = cli::Args::parse();
    println!("{:#?}", args);

    match args.mode {
        cli::Mode::Install => {
            println!("Installing scoop...");
            scoop::install_scoop();

            println!("Installing programs...");
            scoop::install_programs();
        }

        cli::Mode::Uninstall => {
            println!("Uninstalling scoop and all programs...");
            scoop::uninstall_scoop();
        }
    }

    println!("Success! Program will end in 3 seconds");
    let pause = time::Duration::from_secs(3);
    thread::sleep(pause);
}
