use iced::{executor, Command, Container, Element, Length, Application, Settings, Image};

/// Display a file to the user, this can either be an image file or a text file
pub fn display(path: String) {
    Display::run(Settings::with_flags(path));
}

struct Display {
    path: String,
}

impl Application for Display {
    type Executor = executor::Null;
    type Message = ();
    type Flags = String;

    fn new(_flags: Self::Flags) -> (Display, Command<Self::Message>) {
        (Display { path: _flags }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Template Searcher")
    }

    fn update(&mut self, _message: ()) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<()> {
        let img = Image::new(&self.path)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(img)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
