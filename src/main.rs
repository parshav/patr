use std::str;
use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::io::prelude::*;

use serde_json::{Result, Value};
use serde_derive::{Serialize, Deserialize};

// Is there a different way to store config, even from writing down config?
// Separate out the writing function

fn main() {

    let current_directory = env::current_dir()
        .expect("Error in current directory")
        .to_str()
        .expect("Error in converting to string");

    // check for current diredctory
    // Read argument
    // start the writing -> use read [option] or rust built in
    // write out to file, file name should be title ?

    clear_screen();

	let title = get_title();

    let input = read_input_buffer();

    output_to_file(input, title);
}

// Get title from arg else give a default name maybe.
fn get_title() -> String {
	 // maybe better way to d this ?
    let title: String = env::args()
    	.into_iter()
    	.skip(1)
    	.nth(0)
    	.unwrap_or_else(|| {
    		println!("Using default title.");
    		String::from("default.txt")
    	});

    println!("Title is {}", title);

    return title;
}

// Reads input buffer till delim '-'
fn read_input_buffer() -> Vec<u8> {

	let mut buffer = vec![];
	let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b'-', &mut buffer)
    	.expect("Error in read until");

   	return buffer;
}

fn output_to_file(content: Vec<u8>, file_name: String) {
	let mut buffer = File::create(file_name).expect("Error creating file");
	buffer.write_all(&content).expect("Error writing to file");
}

fn clear_screen() {
	print!("\x1B[2J\x1B[1;1H");
}

/// unused 

// checks for config, if exists, use that delim.
// else create new,
fn load_config() -> Config {
    Config {
        name: String::from("Notebook"),
        delim: 'a',
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    delim: char,
}

impl Config {

	fn store_config(&self) {
		let json = serde_json::to_string(&self).expect("Error converting struct to json");
		println!("store json : {}", json);
	}
}
/*
TODO
- patr day5 // patr [title]
    - if patr not initialized, initialize.
    - patr needs deliminator (currently)
- Rethink the git commands usage at all.
- Chain the notes ?
*/
