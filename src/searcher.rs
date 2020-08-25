use web_view::*;
use snailquote::escape;

pub fn start_search() -> () {
    println!("starting search");

    web_view::builder()
        .title(crate::NAME)
        .content(Content::Html(include_str!("display.html")))
        .size(852, 480)
        .resizable(false)
        .frameless(true)
        .user_data(())
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

fn invoke_handler(webview: &mut WebView<()>, arg: &str) -> WVResult {
    println!("recieved arg: {}", arg);

    if arg == "init" {
        webview.eval(&format!("window.searchDir = {}", escape(&crate::config::get("path")))).expect("unable to init window");
        webview.set_fullscreen(true);
    }

    else if arg == "exit" {
        webview.exit();
    }

    Ok(())
}
