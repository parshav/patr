extern crate chrono;

use std::io::{stdin, Read};
use std::env;
use std::fs;

fn main() {
    let mut title = String::new();
    let mut text = String::new();

    let _time = chrono::prelude::Local::now().format("%Y-%m-%d %H:%M:%S");
    let _current_dir = env::current_dir();
    let fileName = String::from("touched");

    println!("Journal Entry");
    println!("Title for today ?");

    stdin().read_line(&mut title)
        .expect("Could not write into title");

    println!("What do you want to write for the day ?");

    stdin().read_line(&mut text)
        .expect("Could not write into text");

    println!("Journal Saved");

    fs::write(fileName, text)
        .expect("Error writing to file");
}