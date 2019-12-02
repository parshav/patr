extern crate chrono;

use std::io::stdin;
use std::env;
use std::fs;

fn main() {

    let mut text = String::new();

    let time = chrono::prelude::Local::now().format("%Y-%m-%d-%H:%M:%S");
    let current_dir = env::current_dir();

    let title = title_handler(env::args().collect());

    println!("debug titel is {}", title);

    println!("debug Current directory {}", current_dir.unwrap().to_string_lossy());

    println!("Journal Entry");

    let file_name = String::from(format!("{}", time));

    println!("What do you want to write for the day ?");

    stdin().read_line(&mut text)
        .expect("Could not write into text");

    let text = format!("{}\n{}", title, text);

    println!("Journal Saved");

    fs::write(file_name, text)
        .expect("Error writing to file");
}

fn title_handler(args: Vec<String>) -> String {
    if args.len() > 1 {
        match args[1].as_ref() {
            "-t" => 
            if args.len() > 2 {
                args[2].to_string()
            } else {
                println!("Forgot to supply title !!");
                prompt_title()
            },
            "-h" => String::from("You have asked for help, ha ! Use -t \"[INSTER_TITLE]\""),
            _ => String::from("Invalid argument, try -h for help")
        }
    } else {
        prompt_title()
    }
}

fn prompt_title() -> String {

    let mut title = String::new();

    println!("Enter the title");
    stdin().read_line(&mut title)
        .expect("Error in title");

    return title;
}

/*
TODO
- Rethink the git commands usage at all.
- Chain the notes ?
*/
