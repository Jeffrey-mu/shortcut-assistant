use std::{thread, time::Duration};

#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY,
    VK_CONTROL, VK_MENU, VK_LMENU, VK_SHIFT, VK_LWIN, VK_ESCAPE, VK_RETURN, VK_SPACE, VK_TAB,
    VK_BACK, VK_DELETE, VK_HOME, VK_END, VK_PRIOR, VK_NEXT, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT,
    VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
    KEYBD_EVENT_FLAGS, MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(target_os = "windows")]
fn send_vk(vk: VIRTUAL_KEY, press: bool) {
    unsafe {
        let scan = MapVirtualKeyW(vk.0 as u32, MAPVK_VK_TO_VSC) as u16;
        let mut flags = if press { KEYBD_EVENT_FLAGS(0) } else { KEYEVENTF_KEYUP };
        
        let mut input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: scan,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                }
            }
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

#[tauri::command]
fn simulate_keys(keys: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let parts: Vec<&str> = keys.split('+').collect();
        let mut modifiers = Vec::new();
        let mut main_vk = None;

        for part in parts {
            let k = part.to_lowercase();
            match k.as_str() {
                "ctrl" => modifiers.push(VK_CONTROL),
                "alt" => modifiers.push(VK_LMENU), // 明确使用左 Alt
                "shift" => modifiers.push(VK_SHIFT),
                "command" | "win" | "meta" => modifiers.push(VK_LWIN),
                _ => {
                    if k.len() == 1 {
                        let ch = k.chars().next().unwrap();
                        if ch.is_ascii_digit() {
                            main_vk = Some(VIRTUAL_KEY(ch as u16)); // '0' is 0x30
                        } else if ch.is_ascii_alphabetic() {
                            let up = ch.to_ascii_uppercase();
                            main_vk = Some(VIRTUAL_KEY(up as u16)); // 'A' is 0x41
                        } else {
                            // 其他符号键暂时转大写对应
                            main_vk = Some(VIRTUAL_KEY(ch.to_ascii_uppercase() as u16));
                        }
                    } else {
                        main_vk = Some(match k.as_str() {
                            "f1" => VK_F1,
                            "f2" => VK_F2,
                            "f3" => VK_F3,
                            "f4" => VK_F4,
                            "f5" => VK_F5,
                            "f6" => VK_F6,
                            "f7" => VK_F7,
                            "f8" => VK_F8,
                            "f9" => VK_F9,
                            "f10" => VK_F10,
                            "f11" => VK_F11,
                            "f12" => VK_F12,
                            "esc" | "escape" => VK_ESCAPE,
                            "enter" => VK_RETURN,
                            "space" => VK_SPACE,
                            "tab" => VK_TAB,
                            "backspace" => VK_BACK,
                            "delete" => VK_DELETE,
                            "home" => VK_HOME,
                            "end" => VK_END,
                            "pageup" => VK_PRIOR,
                            "pagedown" => VK_NEXT,
                            "up" => VK_UP,
                            "down" => VK_DOWN,
                            "left" => VK_LEFT,
                            "right" => VK_RIGHT,
                            _ => return Err(format!("不支持的按键: {}", k)),
                        });
                    }
                }
            }
        }

        if main_vk.is_none() {
            return Err("未识别到主按键".to_string());
        }
        let main_vk = main_vk.unwrap();

        // 1. 按下修饰键
        for &modifier in &modifiers {
            send_vk(modifier, true);
        }

        // 短暂延迟保证修饰键状态被系统记录
        if !modifiers.is_empty() {
            thread::sleep(Duration::from_millis(20));
        }

        // 2. 按下主键
        send_vk(main_vk, true);
        
        // 保持按下状态，确保目标软件事件循环能捕获
        thread::sleep(Duration::from_millis(20));

        // 3. 释放主键
        send_vk(main_vk, false);

        // 4. 释放修饰键 (逆序)
        for &modifier in modifiers.iter().rev() {
            send_vk(modifier, false);
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("目前仅支持 Windows 系统".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag"])))
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
                use tauri::menu::{Menu, MenuItem};
                use tauri::Manager;

                let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
                let quit_i = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, simulate_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
