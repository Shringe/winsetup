use clap::Parser;
use scoop as scoop_root;
use std::{io, thread, time};

mod cli;
mod scoop;

const DIVIDER_WIDTH: usize = 20;
const DIVIDER_SYMBOL: &str = "=";

fn prompt_options() -> String {
    let text = "Type a number for each option to perform.\n\
        If you want both [1] and [2], then type 12.\n\
         - [1] Install scoop\n\
         - [2] Update scoop and programs\n\
         - [3] Install games\n\
         - [4] Install academic tools\n\
         - [5] Install programming tools\n\
        Type your answer below then hit enter:";
    println!("{}", text);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn print_divider() {
    let divider = DIVIDER_SYMBOL.repeat(DIVIDER_WIDTH);
    println!("{}", divider);
}

fn main() {
    let args = cli::Args::parse();
    if args.dryrun {
        println!("{:#?}", args);
    }

    let answer = prompt_options();
    if args.dryrun {
        println!("Input: {}", answer);
    }

    print_divider();

    let scoop = scoop::Scoop { cmd_args: &args };
    let mut buckets = Vec::new();
    let mut programs = Vec::new();

    if answer.contains('3') {
        println!("Preparing to install games...");
        buckets.extend(scoop_root::GAME_BUCKETS);
        programs.extend(scoop_root::GAME_PROGRAMS);
    }

    if answer.contains('4') {
        println!("Preparing to install academic software...");
        buckets.extend(scoop_root::ACADEMIC_BUCKETS);
        programs.extend(scoop_root::ACADEMIC_PROGRAMS);
    }

    if answer.contains('5') {
        println!("Preparing to install programming software...");
        buckets.extend(scoop_root::PROGRAMMING_BUCKETS);
        programs.extend(scoop_root::PROGRAMMING_PROGRAMS);
    }

    if answer.contains('1') {
        print_divider();
        println!("Installing scoop if it's not available...");
        scoop.install_scoop();
        print_divider();
    }

    if answer.contains('2') {
        println!("Updating scoop if it's available...");
        scoop.update_scoop();
        print_divider();
    }

    println!("Installing all buckets...");
    scoop.add_buckets(&buckets);
    print_divider();

    println!("Installing all programs...");
    scoop.add_programs(&programs);
    print_divider();

    println!("Success! Program will end in 3 seconds");
    let pause = time::Duration::from_secs(3);
    if !args.dryrun {
        thread::sleep(pause);
    }
}
