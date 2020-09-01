
fn main() {
    cc::Build::new()
        .file("src/window.c")
        .compile("window");
}