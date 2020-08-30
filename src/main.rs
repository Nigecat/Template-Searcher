mod window;
use window::Window;

fn main() {
    let win = Window::new("Template Searcher");
    win.display("resources/banner.png".to_string());
    win.main_loop();
    win.destroy();
}