pub struct Window { }

impl Window {
    /// Create a new window
    pub fn new() -> Self {
        Window { }
    }

    /// Display an image on this window
    pub fn display(&self, path: String) {
        println!("displaying image: {}", path);
    }

    /// Destroy this window
    pub fn destroy(&self) {
        println!("destroying window...");
    }
}