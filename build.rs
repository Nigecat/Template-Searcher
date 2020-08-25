use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=\"src/display.html\"");

    // Delete the old content if it exists
    if Path::new("src/display.rs").exists() {
        fs::remove_file("src/display.rs");
    }

    // Read the html content and add quotes to either side
    let content = ["r#\"".to_string(), fs::read_to_string("src/display.html").expect("src/display.html not found"), "\"#".to_string()].join("");

    // Write it out
    fs::write("src/display.rs", content).expect("could not write html content");
}