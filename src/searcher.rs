use web_view::*;

pub fn start_search(path: String) -> () {
    println!("starting search at {}", path);
	
    web_view::builder()
        .title(crate::NAME)
        .content(Content::Html(include!("display.rs")))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
