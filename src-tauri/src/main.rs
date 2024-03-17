// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/**
 * Code in this file strongly inspired by Siddharth Jha
 * https://github.com/clearlysid/modern-clippy/blob/main/src-tauri/src/main.rs
 * (used under MIT license)
 * */

extern crate dirs;
use std::{fs::File, io::Read};
use tauri::{
  App,
  AppHandle,
  CustomMenuItem,
  GlobalShortcutManager,
  Manager,
  SystemTray,
  SystemTrayEvent,
  SystemTrayMenu,
};

const GLOBAL_SHORTCUT_OPEN: &str = "CmdOrCtrl+,";
const GLOBAL_SHORTCUT_QUIT: &str = "CmdOrCtrl+Q";
const INIT_MESSAGE: &str = "\
<strong>Greetings, {username}!</strong> I am Clippy, your new AI companion. You might have arrived here \
after watching that one video from FlyTech Videos. I hope you enjoyed that video! With that said, I have \
to disappoint you - I am actually not as smart as the video made it seem. All my responses are prerecorded \
in script files. You can download the script from the video at <a href=\"https://flies.sh/evil-clippy-script\">\
https://flies.sh/evil-clippy-script</a>.
<ul>
<li>Place it in %APPDATA%\\EvilClippy\\scripts.</li>
<li>Load it by typing <em>/load filename</em> into the chat bar (e.g. /load script1.tsv). If it was successful, the chat window will clear.</li>
<li>Note that your file has to match the same format (tab-separated rows, \
check out the \"evil clippy script\" from above to understand the format).</li>
<li>A list of possible Clippy actions can be found at \
<a href=\"https://mklab.eu.org/clippy/\">https://mklab.eu.org/clippy/</a>.</li>
</ul>

With that said, I would appreciate if you could like and share the original video. Thank you!
<br><br>
All the best,
<br>
Fly
<br><br>
P.S. Source code at <a href=\"https://flies.sh/evil-clippy-github\">https://flies.sh/evil-clippy-github</a>
";

const CLIPPY_SCRIPT_DIR: &str = "\\EvilClippy\\scripts\\";

#[tauri::command]
fn obtain_init_message() -> String {
    let name = whoami::realname();
    return INIT_MESSAGE.replace("{username}", &name);
}

#[tauri::command]
fn load_script(script_command: &str) -> String {
    if !script_command.starts_with("/load ") {
        return "".to_owned();
    }
    let file_name = &script_command[6..];
    if file_name.contains("/") || file_name.contains("\\") {
        return "".to_owned();
    }

    let appdata_path = dirs::config_dir().expect("could not unwrap appdata dir");
    let appdata_dir: &str = appdata_path.to_str().expect("could not convert appdata_dir");

    let path = appdata_dir.to_owned() + CLIPPY_SCRIPT_DIR + file_name;
    let file = File::open(path);
    if file.is_err() {
        return "".to_owned();
    }

    let mut contents = String::new();
    let read_result = file.unwrap().read_to_string(&mut contents);
    if read_result.is_err() {
        return "".to_owned();
    }

    return contents;
}

fn configure_tray_icon() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("clippy", "Clippy").accelerator(GLOBAL_SHORTCUT_OPEN))
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator(GLOBAL_SHORTCUT_QUIT)),
    )
}

fn toggle_window_visibility(app_handle: &AppHandle) {
  if let Some(window) = app_handle.get_window("main") {
      match window.is_visible() {
          Ok(true) => window
              .hide()
              .unwrap_or_else(|e| println!("Failed to hide window: {}", e)),
          Ok(false) => window
              .show()
              .unwrap_or_else(|e| println!("Failed to show window: {}", e)),
          Err(e) => println!("Failed to check window visibility: {}", e),
      }
  } else {
      println!("Window not found");
  }
}

fn register_global_shortcut(app: App) {
  app.run(|_app_handle, event| match event {
    tauri::RunEvent::Ready => {
        let app_handle_clone = _app_handle.clone();
        app_handle_clone
            .global_shortcut_manager()
            .register(GLOBAL_SHORTCUT_OPEN, move || toggle_window_visibility(&app_handle_clone))
            .unwrap();
    }
    _ => {}
  });
}

fn main() {
    let app = tauri::Builder::default()
        .system_tray(configure_tray_icon())
        .on_system_tray_event(|app_handle, event| match event {
            SystemTrayEvent::LeftClick { .. } => toggle_window_visibility(&app_handle),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "clippy" => toggle_window_visibility(&app_handle),
                "quit" => std::process::exit(0),
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![obtain_init_message, load_script])
        .build(tauri::generate_context!())
        .expect("Error while building Clippy");

    register_global_shortcut(app);
}
