use std::str;
use std::env;
use std::io::{self, BufRead};

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

    let input = read_input_buffer();

}

// Get title from arg else give a default name maybe.
fn get_title() -> String {
	 // maybe better way to d this ?
    let title: String = env::args()
    	.into_iter()
    	.skip(1)
    	.nth(0)
    	.expect("Error in getting argument");

    println!("Title is {}", title);

    return title;
}

// Reads input buffer till delim '-'
fn read_input_buffer() -> String {

	let mut buffer = vec![];
	let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b'-', &mut buffer)
    	.expect("Error in read until");

   	let buffer = str::from_utf8(&buffer).expect("error in utf8");

   	return buffer.to_owned();
}

// checks for config, if exists, use that delim.
// else create new,
fn config_check_for_delim() -> Config {
    Config {
        name: String::from("Notebook"),
        delim: 'a',
    }
}

struct Config {
    name: String,
    delim: char,
}
/*
TODO
- patr day5 // patr [title]
    - if patr not initialized, initialize.
    - patr needs deliminator (currently)
- Rethink the git commands usage at all.
- Chain the notes ?
*/
