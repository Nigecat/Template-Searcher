mod window;
use window::Window;

fn main() {
    let win = Window::new();
    win.display();
    win.destroy();
}