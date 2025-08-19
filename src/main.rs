use clap::Parser;

mod cli;
mod scoop;

fn main() {
    let args = cli::Args::parse();
    println!("{:#?}", args);

    if !scoop::is_program_in_path("scoop") {
        println!("Scoop not found.");
        println!("Installing scoop...");

        if !args.dryrun {
            scoop::install_scoop();
        }
    }
}
