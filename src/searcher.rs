use web_view::*;
use snailquote::escape;

pub fn start_search() -> () {
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
    println!("recieved arg: {}", arg);

    if arg == "init" {
        webview.set_fullscreen(true);
        webview.eval(&format!("setup({})", escape(&crate::config::get("path")))).expect("unable to init window");
    }

    else if arg == "exit" {
        webview.exit();
    }

    Ok(())
}
