use std::env;
use std::fs;
use std::path::Path;

// Is there a different way o store config.
// Separate out the writing function

fn main() {
    let current_directory = env::current_dir()
        .expect("Error in current directory")
        .to_str()
        .expect("Error in converting to string");
    // check for current diredctory

    //let config_exists = Path::new(format!("{}/.patr/config", current_directory).as_str());
}

// checks for config, if exists, use that delim.
// else create new,
fn config_check_for_delim() -> Config {
	Config {
		name: String::from("hi"),
		delim: 'a'
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
