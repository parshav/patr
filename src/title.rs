use std::io::stdin;

pub fn title_handler(args: Vec<String>) -> String {
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