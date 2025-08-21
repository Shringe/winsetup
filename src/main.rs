use colored::{ColoredString, Colorize};
use scoop as scoop_root;
use std::{io, process::exit, thread, time};

mod config;
mod scoop;
mod style;

const DIVIDER_WIDTH: usize = 20;
const DIVIDER_SYMBOL: &str = "=";

fn prompt_options() -> String {
    let warning = "The first time running this program, you must select 1, and then run this program again.\n\
        This will set everything else up for installation.";

    let body = "Type a number for each option to perform.\n\
        If you want both [1] and [2], then type 12.\n\
         - [1] Install scoop\n\
         - [2] Update scoop and programs\n\
         - [3] Install games\n\
         - [4] Install academic tools\n\
         - [5] Install programming tools\n\
         - [6] Install personal configuration (not recommended)\n\
         - [remove] Uninstall everything\n\
        Type your answer below then hit enter:";

    println!("{}\n{}\n{}", warning, make_divider(), body);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()
}

fn make_divider() -> ColoredString {
    DIVIDER_SYMBOL.repeat(DIVIDER_WIDTH).blue().bold()
}

fn print_divider() {
    println!("{}", make_divider());
}

fn finish_program() -> ! {
    print_divider();
    println!("Success! Program will end in 3 seconds");
    let pause = time::Duration::from_secs(3);
    thread::sleep(pause);
    exit(0);
}

fn main() {
    let answer = prompt_options();
    let dryrun = cfg!(debug_assertions);
    let scoop = scoop::Scoop { dryrun };
    let config = config::Config::new(dryrun);

    if dryrun {
        println!("Input: {}", answer);
    }

    print_divider();

    if answer == "remove" {
        status!("Uninstalling scoop and all software...");
        scoop.uninstall();
        config.uninstall();
        finish_program();
    }

    let mut buckets = Vec::new();
    let mut programs = Vec::new();

    if answer.contains('3') {
        status!("Preparing to install games...");
        buckets.extend(scoop_root::GAME_BUCKETS);
        programs.extend(scoop_root::GAME_PROGRAMS);
    }

    if answer.contains('4') {
        status!("Preparing to install academic software...");
        buckets.extend(scoop_root::ACADEMIC_BUCKETS);
        programs.extend(scoop_root::ACADEMIC_PROGRAMS);
    }

    if answer.contains('5') {
        status!("Preparing to install programming software...");
        buckets.extend(scoop_root::PROGRAMMING_BUCKETS);
        programs.extend(scoop_root::PROGRAMMING_PROGRAMS);
    }

    if answer.contains('6') {
        status!("Setting up config...");
        buckets.extend(scoop_root::PERSONAL_BUCKETS);
        programs.extend(scoop_root::PERSONAL_PROGRAMS);
        config.install();
    }

    print_divider();

    if answer.contains('1') {
        status!("Installing scoop if it's not available...");
        scoop.install();
        print_divider();
    }

    if answer.contains('2') {
        status!("Updating scoop if it's available...");
        scoop.update();
        print_divider();
    }

    buckets.sort();
    buckets.dedup();
    programs.sort();
    programs.dedup();

    status!("Installing all buckets...");
    scoop.add_buckets(&buckets);
    print_divider();

    status!("Installing all programs...");
    scoop.add_programs(&programs);

    finish_program();
}
