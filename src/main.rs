use display::display;
mod display;

pub fn main() {
    display(String::from(std::env::args().nth(1).unwrap()));
}
