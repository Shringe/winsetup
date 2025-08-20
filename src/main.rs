use scoop as scoop_root;
use std::{io, process::exit, thread, time};

mod scoop;

const DIVIDER_WIDTH: usize = 20;
const DIVIDER_SYMBOL: &str = "=";

fn prompt_options() -> String {
    let text = "Type a number for each option to perform.\n\
        If you want both [1] and [2], then type 12.\n\
        The first time arount you MUST only type 1.\n\
        Afterwards you can choose the software you want.\n\
         - [1] Install scoop\n\
         - [2] Update scoop and programs\n\
         - [3] Install games\n\
         - [4] Install academic tools\n\
         - [5] Install programming tools\n\
         - [remove] Uninstall everything\n\
        Type your answer below then hit enter:";
    println!("{}", text);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()
}

fn print_divider() {
    let divider = DIVIDER_SYMBOL.repeat(DIVIDER_WIDTH);
    println!("{}", divider);
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
    println!("Input: {}", answer);

    print_divider();

    let scoop = scoop::Scoop { dryrun: false };

    if answer == "remove" {
        println!("Uninstalling scoop and all software...");
        scoop.uninstall();
        finish_program();
    }

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

    print_divider();

    if answer.contains('1') {
        println!("Installing scoop if it's not available...");
        scoop.install();
        print_divider();
    }

    if answer.contains('2') {
        println!("Updating scoop if it's available...");
        scoop.update();
        print_divider();
    }

    buckets.sort();
    buckets.dedup();
    programs.sort();
    programs.dedup();

    println!("Installing all buckets...");
    scoop.add_buckets(&buckets);
    print_divider();

    println!("Installing all programs...");
    scoop.add_programs(&programs);

    finish_program();
}
