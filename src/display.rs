use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use image::image_dimensions;

pub fn display_file(path: &str) {
    let img = image_dimensions(path);

    match img {
        Ok(dimensions) => display_image(dimensions, path),
        Err(_e) => display_text(path)
    }

}

fn display_image(dimensions: (u32, u32), path: &str) {
    Command::new("helper.exe")
        .args(&["image", &dimensions.0.to_string(), &dimensions.1.to_string(), path])
        .status()
        .unwrap();
}

fn display_text(path: &str) {
    let mut content = Vec::new();
    File::open(path).unwrap().read_to_end(&mut content).unwrap();
    Command::new("helper.exe")
        .args(&["text", &String::from_utf8_lossy(&content)])
        .status()
        .unwrap();
}
