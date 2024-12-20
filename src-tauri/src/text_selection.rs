use tauri::command;
use enigo::*;
use active_win_pos_rs::get_active_window;

#[command]
pub fn get_selected_text() -> String {
    // Get the current window info
    if let Ok(window) = get_active_window() {
        // Create a new enigo instance
        let mut enigo = Enigo::new();
        
        // Save current clipboard content
        let old_clipboard = arboard::Clipboard::new()
            .and_then(|mut clipboard| clipboard.get_text())
            .unwrap_or_default();
        
        // Small delay before copying
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Try to copy multiple times to ensure we get the text
        let mut selected_text = String::new();
        for _ in 0..3 {
            // Simulate Ctrl+C/Cmd+C to copy selected text
            if cfg!(target_os = "macos") {
                enigo.key_down(Key::Meta);
                enigo.key_click(Key::Layout('c'));
                enigo.key_up(Key::Meta);
            } else {
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('c'));
                enigo.key_up(Key::Control);
            }
            
            // Small delay after copying
            std::thread::sleep(std::time::Duration::from_millis(100));
            
            // Get the selected text from clipboard
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    if !text.is_empty() && text != old_clipboard {
                        selected_text = text;
                        break;
                    }
                }
            }
        }
        
        // Restore old clipboard content
        if !old_clipboard.is_empty() {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text(old_clipboard);
            }
        }
        
        selected_text
    } else {
        String::new()
    }
}
