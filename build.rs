use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.compile().unwrap();

    // Rebuild our helper autohotkey script if it has changed
    println!("cargo:rerun-if-changed=src/helper.ahk");
    Command::new("Ahk2Exe.exe")
        .args(&["/in", "src/helper.ahk", "/out", Path::new(&env::var("OUT_DIR").unwrap()).join("helper.exe").to_str().unwrap(), "/icon", "resources/icon.ico"])
        .status()
        .expect("unable to build src/helper.ahk (ensure you have Ahk2Exe.exe in your PATH)");
}
