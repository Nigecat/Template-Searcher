use std::env;
mod config;

const NAME: &str = "Template Searcher";

fn parse_args() -> () {
    // Collect all the arguments we got
    let mut args: Vec<String> = env::args().collect();
    // Remove the first argument as this is the binary file path
    args.remove(0);
    // Error if we don't have an even number of arguments
    if args.len() % 2 != 0 {
        panic!("there must be an even number of arguments (e.g --shortcut control+shift+c)");
    }
    // Loop through each argument to check if any of our config flags have been set
    for i in (0..args.len()).step_by(2) {
        if args[i] == "--shortcut" {
            config::set("shortcut", args[i + 1].clone());
        } else if args[i] == "--path" {
            config::set("path", args[i + 1].clone())
        } else {
            panic!("unknown flag")
        }
    }
}

fn main() {
    parse_args();
}
