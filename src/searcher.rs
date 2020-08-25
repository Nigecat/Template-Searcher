use web_view::*;

pub fn start_search(path: String) -> () {
    println!("starting search at {}", path);
    
    web_view::builder()
        .title(crate::NAME)
        .content(Content::Html(include_str!("display.html")))
        .size(852, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

fn invoke_handler(webview: &mut WebView<()>, arg: &str) -> WVResult {
    if arg == "init" {
        webview.set_fullscreen(true);
    }

    else if arg == "exit" {
        webview.exit();
    }

    Ok(())
}
