extern crate chrono;

use std::io::stdin;
use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

fn main() {
    let mut title = String::new();
    let mut text = String::new();

    let time = chrono::prelude::Local::now().format("%Y-%m-%d-%H:%M:%S");
    let current_dir = env::current_dir();

    println!("debug Current directory {}", current_dir.unwrap().to_string_lossy());

    println!("Journal Entry");
    println!("Title for today ?");

    stdin().read_line(&mut title)
        .expect("Could not write into title");

    let file_name = String::from(format!("{}", time));

    println!("What do you want to write for the day ?");

    stdin().read_line(&mut text)
        .expect("Could not write into text");

    let text = format!("{}\n{}", title, text);

    println!("Journal Saved");

    fs::write(file_name, text)
        .expect("Error writing to file");

    if Path::new("./.git").exists() {
        println!("Directory exists");
    } else {
        println!("Directory does not exist");
    }

    commit_and_push(&title)
}

fn commit_and_push(msg: &String) {
    Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("Error in git add .")
        .wait();

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .spawn()
        .expect("Error in git commit")
        .wait();

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .spawn()
        .expect("Error in git push")
        .wait();
}