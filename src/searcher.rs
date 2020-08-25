use web_view::*;

static mut SEARCH: String = String::new();

pub fn start_search() {
    println!("starting search");

    web_view::builder()
        .title(crate::NAME)
        .content(Content::Html(include_str!("display.html")))
        .size(852, 480)
        .resizable(false)
       // .frameless(true)
        .user_data(())
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

fn invoke_handler(webview: &mut WebView<()>, arg: &str) -> WVResult {
    unsafe {
        println!("recieved arg: {}", arg);
    
        if arg == "init" {
           // webview.set_fullscreen(true);
           webview.eval("document.body.focus()").ok();
        }
    
        else if arg.starts_with("key-") {
            if arg == "key-Spacebar" {
                SEARCH.push(' ');
            }
            
            else if arg == "key-Backspace" {
                // Only remove a character if we have any characters
                if SEARCH.chars().count() > 0 {
                    // Slice all the characters except the last one
                    SEARCH = SEARCH[..SEARCH.chars().count() - 1].to_string();
                }
            } 

            // Cancel the search
            else if arg == "key-Esc" {
                webview.exit();
            }

            // Send the current match
            else if arg == "key-Enter" {
                println!("sending search: {}", SEARCH);
                webview.exit();
            }
            
            // Only add it to our search if we have exactly 5 characters
            // This will ignore any other control characters like shift
            else if arg.chars().count() == 5 {
                SEARCH.push(arg.chars().last().unwrap());
            }
        }
    
        else if arg == "exit" {
            webview.exit();
        }

        println!("current search: {}", SEARCH);
    
        Ok(())
    }
}
