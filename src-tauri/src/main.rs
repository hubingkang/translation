// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod text_selection;

// use text_selection::get_selected_text;
mod clip;
mod command;

fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![get_selected_text])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::Emitter;
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["ctrl+d", "alt+space"])?
                        .with_handler(|app, shortcut, event| {
                            if event.state == ShortcutState::Pressed  {
                                if shortcut.matches(Modifiers::CONTROL, Code::KeyD) {
                                    let _ = app.emit("shortcut-event", "Ctrl+D triggered");
                                }
                                if shortcut.matches(Modifiers::ALT, Code::Space) {
                                    let _ = app.emit("shortcut-event", "Alt+Space triggered");
                                }
                            }
                        })
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command::get_selection_text])
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    // use enigo::{
    //     Direction::{Click, Press, Release},
    //     Enigo, Key, Keyboard, Settings,
    // };
    // use std::thread;
    // use std::time::Duration;
    // thread::sleep(Duration::from_secs(2));
    // let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // // write text
    // enigo
    //     .text("Hello World! here is a lot of text  ❤️")
    //     .unwrap();

    // // select all
    // enigo.key(Key::Command, Press).unwrap();
    // enigo.key(Key::Unicode('c'), Click).unwrap();
    // enigo.key(Key::Command, Release).unwrap();

    // println!("Press any key to continue...");

}
