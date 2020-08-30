mod window;
use window::Window;

fn main() {
    let win = Window::new();
    win.display("resources/banner.png".to_string());
    win.destroy();
}