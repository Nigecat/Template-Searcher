use std::process;

#[cfg(windows)]
#[allow(unreachable_code)]
pub fn create_tray() -> Result<(), systray::Error>  {
    let mut tray = systray::Application::new().unwrap();
    tray.set_icon_from_file("resources/icon.ico").expect("unable to load tray icon");

    tray.add_menu_item("Restart", |_| {
        println!("restart");
        Ok::<_, systray::Error>(())
    })?;

    tray.add_menu_item("Exit", |_| {
        process::exit(0);
        Ok::<_, systray::Error>(())
    })?;

    tray.wait_for_message()?;
    Ok(())
}