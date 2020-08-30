use orbtk::prelude::*;

pub fn show_image(path: &'static str) {
    Application::new()
        .window(move |ctx| {
            let size = imagesize::size(path).expect("unable to parse image size");
            Window::new()
                .title("Template Searcher")
                .position((0.0, 0.0))
                .resizeable(false)
                .borderless(true)
                .active(true)
                .always_on_top(true)
                .size(size.width as u32, size.height as u32)
                .child(ImageWidget::new()
                    .image(path)
                    .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
