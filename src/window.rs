use orbtk::prelude::*;

pub fn show_image(path: &'static str) {
    Application::new()
        .window(move |ctx| {
            Window::new()
                .title("Template Searcher")
                .position((0.0, 0.0))
                .size(852.0, 480.0)
                .opacity(0.5)
                .child(ImageWidget::new()
                    .image(path)
                    .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
