extern {
    fn create_window();
}

fn main() {
    unsafe {
        create_window();
    }
}
