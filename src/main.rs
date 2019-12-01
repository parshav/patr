extern crate chrono;

use std::io::stdin;
use std::env;
use std::fs;

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

}

/*
TODO
- Rethink the git commands usage at all.
- Chain the notes ?
*/
