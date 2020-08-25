use std::env;
use std::thread;
mod config;
mod searcher;
mod matcher;

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

    let shortcut = config::get("shortcut");

    // Calculate our modifiers
    let mut modifiers = 0;
    if shortcut.contains("control") { modifiers += hotkey::modifiers::CONTROL; }
    if shortcut.contains("shift") { modifiers += hotkey::modifiers::SHIFT; }
    if shortcut.contains("windows") { modifiers += hotkey::modifiers::SUPER; }
    if shortcut.contains("alt") { modifiers += hotkey::modifiers::ALT; }

    // Register our hotkey
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        modifiers,
        // Convert the last character of the shortcut string to an ascii key code
        shortcut.chars().last().unwrap().to_ascii_uppercase() as u32,
        || {
            thread::spawn(|| {
                searcher::start_search();
            });
        },
    ).expect("could not create hotkey");
    hk.listen();
}