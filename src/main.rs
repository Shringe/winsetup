use clap::Parser;
use std::{thread, time};

mod cli;
mod scoop;

fn main() {
    let args = cli::Args::parse();
    println!("{:#?}", args);

    println!("Installing scoop...");
    if !args.dryrun {
        scoop::install_scoop();
    }

    println!("Success! Program will end in 3 seconds");
    let pause = time::Duration::from_secs(3);
    thread::sleep(pause);
}
