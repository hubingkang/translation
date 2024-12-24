// #[cfg(target_os = "macos")]
// pub fn read_text() -> Result<String, String> {
//   use std::process::Command;
//   use std::thread;
//   use std::time::Duration;
 
//     // 模拟 Command+C 以复制当前选中的内容
//     let script = r#"
//         tell application "System Events"
//             keystroke "c" using {command down}
//         end tell
//     "#;

//     Command::new("osascript")
//         .arg("-e")
//         .arg(script)
//         .output()
//         .expect("Failed to execute AppleScript");

//     thread::sleep(Duration::from_millis(300));
//     // 从剪贴板读取复制的内容
//     let output = Command::new("pbpaste")
//         .output()
//         .expect("Failed to execute pbpaste");

//     Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
// }

#[cfg(target_os = "macos")]
pub fn read_text() -> Result<String, String> {
  use selection::get_text;

  let text = get_text();
  
  println!("selection ====> text: {}", text);

  if text.is_empty() {
    return Err("No text selected".to_string());
  }

  return Ok(text);
}

// #[cfg(target_os = "macos")]
// pub fn read_text() -> Result<String, String> {
//   use enigo::{
//     Direction::{Click, Press, Release},
//     Enigo, Key, Keyboard, Settings,
//   };
//   use std::thread;
//   use std::time::Duration;

//   use selection::get_text;

//   let text = get_text();
  
//   println!("selection ====> text: {}", text);

//   println!("准备初始化 Enigo");

//   let settings = Settings::default();
//   let mut enigo = Enigo::new(&settings).map_err(|e| format!("Failed to initialize Enigo: {:?}", e))?;

//   // 给系统一点时间准备
//   // thread::sleep(Duration::from_secs(2));
//   thread::sleep(Duration::from_millis(200));

//   println!("开始执行复制操作");
  
//   // // 按下 Command 键
//   // enigo.key(Key::Command, Press)
//   //     .map_err(|e| format!("Failed to press Command key: {:?}", e))?;
  
//   // thread::sleep(Duration::from_millis(200));

//   // // 按下并释放 A 键
//   // enigo.key(Key::Unicode('a'), Click)
//   //     .map_err(|e| format!("Failed to press A key: {:?}", e))?;
  
//   // thread::sleep(Duration::from_millis(200));
  
//   // enigo.key(Key::Unicode('a'), Release)
//   //     .map_err(|e| format!("Failed to release A key: {:?}", e))?;

//   // thread::sleep(Duration::from_millis(200));

//   // // 释放 Command 键
//   // enigo.key(Key::Command, Release)
//   //     .map_err(|e| format!("Failed to release Command key: {:?}", e))?;

//   // 使用 Result 来处理每个键盘操作
//   let result = (|| -> Result<(), String> {
//     println!("按下 Command 键");
//     enigo.key(Key::Command, Press)
//         .map_err(|e| format!("Command Press 失败: {:?}", e))?;
    
//     // thread::sleep(Duration::from_millis(100));
    
//     println!("按下 C 键");
//     enigo.key(Key::Other(0x08), Click)
//         .map_err(|e| format!("C Click 失败: {:?}", e))?;
    
//     thread::sleep(Duration::from_millis(100));
    
//     println!("释放 Command 键");
//     enigo.key(Key::Command, Release)
//         .map_err(|e| format!("Command Release 失败: {:?}", e))?;
    
//     Ok(())
//   })();

//   if let Err(e) = result {
//       println!("键盘操作失败: {}", e);
//       return Err(e);
//   }

//   thread::sleep(Duration::from_millis(100));

//   println!("等待键盘操作完成");
//   // select all
//   // enigo.key(Key::Command, Press).unwrap();
//   // enigo.key(Key::Unicode('a'), Click).unwrap();
//   // enigo.key(Key::Command, Release).unwrap();

//   println!("Hello, world!");

//   println!("键盘操作完成，等待剪贴板更新");
  
//   // 给剪贴板更多时间更新
//   // thread::sleep(Duration::from_millis(200));
//   // 读取剪贴板内容
//   // read_text_cross()


//   let mut clipboard = arboard::Clipboard::new().unwrap();
//   println!("Clipboard text was: {}", clipboard.get_text().unwrap());
//   Ok(clipboard.get_text().unwrap())

// }

fn read_text_cross() -> Result<String, String> {
  use clipboard::{ClipboardContext, ClipboardProvider};
  
  println!("准备读取剪贴板");
  let mut ctx: ClipboardContext = match ClipboardProvider::new() {
    Ok(ctx) => {
      println!("剪贴板初始化成功");
      ctx
    },
    Err(e) => {
      println!("剪贴板初始化失败: {:?}", e);
      return Err(format!("无法访问剪贴板: {:?}", e));
    }
  };

  // 读取剪贴板内容
  match ctx.get_contents() {
    Ok(text) => {
      println!("成功读取剪贴板内容: {}", text);
      Ok(text)
    },
    Err(e) => {
      println!("读取剪贴板失败: {:?}", e);
      Err(format!("无法读取剪贴板内容: {:?}", e))
    }
  }
}


// enigo = { version = "0.3.0", features = ["serde"] }
