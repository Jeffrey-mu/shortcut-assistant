use std::{thread, time::Duration};

#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation, CGKeyCode};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
#[cfg(target_os = "macos")]
use std::process::Command;
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC, VIRTUAL_KEY,
    VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12,
    VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_HOME, VK_INSERT, VK_LEFT,
    VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SPACE,
    VK_TAB, VK_UP,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(target_os = "windows")]
const KEY_HOLD_MS: u64 = 25;
#[cfg(target_os = "windows")]
const MODIFIER_SETTLE_MS: u64 = 25;
#[cfg(target_os = "macos")]
const MACOS_KEY_HOLD_MS: u64 = 70;
#[cfg(target_os = "macos")]
const MACOS_MODIFIER_SETTLE_MS: u64 = 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Key {
    Character(char),
    Control,
    Alt,
    Shift,
    Meta,
    Escape,
    Enter,
    Space,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    F(u8),
}

#[derive(Debug)]
struct ParsedShortcut {
    modifiers: Vec<Key>,
    main_key: Key,
}

#[tauri::command]
fn simulate_keys(keys: String) -> Result<(), String> {
    let shortcut = parse_shortcut(&keys)?;

    #[cfg(target_os = "windows")]
    {
        simulate_key_combo_windows(&shortcut)
    }

    #[cfg(target_os = "macos")]
    {
        simulate_key_combo_macos(&shortcut)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        Err("目前仅支持 Windows 和 macOS 系统".to_string())
    }
}

fn parse_shortcut(keys: &str) -> Result<ParsedShortcut, String> {
    let mut modifiers = Vec::new();
    let mut main_key = None;

    for part in keys.split('+') {
        let raw = part.trim();
        if raw.is_empty() {
            continue;
        }

        let key = parse_key(raw)?;
        match key {
            Key::Control | Key::Alt | Key::Shift | Key::Meta => {
                if !modifiers.contains(&key) {
                    modifiers.push(key);
                }
            }
            _ => main_key = Some(key),
        }
    }

    let main_key = main_key.ok_or_else(|| "未识别到主按键".to_string())?;
    Ok(ParsedShortcut {
        modifiers,
        main_key,
    })
}

fn parse_key(raw: &str) -> Result<Key, String> {
    let normalized = raw.trim().to_lowercase();

    match normalized.as_str() {
        "ctrl" | "control" => Ok(Key::Control),
        "alt" | "option" => Ok(Key::Alt),
        "shift" => Ok(Key::Shift),
        "command" | "cmd" | "meta" | "win" | "super" => Ok(Key::Meta),
        "esc" | "escape" => Ok(Key::Escape),
        "enter" | "return" => Ok(Key::Enter),
        "space" => Ok(Key::Space),
        "tab" => Ok(Key::Tab),
        "backspace" => Ok(Key::Backspace),
        "delete" | "del" => Ok(Key::Delete),
        "insert" | "ins" => Ok(Key::Insert),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" | "page up" | "pgup" => Ok(Key::PageUp),
        "pagedown" | "page down" | "pgdn" => Ok(Key::PageDown),
        "up" | "arrowup" => Ok(Key::Up),
        "down" | "arrowdown" => Ok(Key::Down),
        "left" | "arrowleft" => Ok(Key::Left),
        "right" | "arrowright" => Ok(Key::Right),
        _ if normalized.starts_with('f') => {
            let number = normalized[1..]
                .parse::<u8>()
                .map_err(|_| format!("不支持的按键: {}", raw))?;
            if (1..=12).contains(&number) {
                Ok(Key::F(number))
            } else {
                Err(format!("不支持的功能键: {}", raw))
            }
        }
        _ if raw.chars().count() == 1 => Ok(Key::Character(raw.chars().next().unwrap())),
        _ => Err(format!("不支持的按键: {}", raw)),
    }
}

#[cfg(target_os = "windows")]
fn simulate_key_combo_windows(shortcut: &ParsedShortcut) -> Result<(), String> {
    let modifiers = shortcut
        .modifiers
        .iter()
        .map(windows_virtual_key)
        .collect::<Result<Vec<_>, _>>()?;
    let main_vk = windows_virtual_key(&shortcut.main_key)?;

    for modifier in &modifiers {
        send_vk(*modifier, true)?;
    }

    if !modifiers.is_empty() {
        thread::sleep(Duration::from_millis(MODIFIER_SETTLE_MS));
    }

    send_vk(main_vk, true)?;
    thread::sleep(Duration::from_millis(KEY_HOLD_MS));
    send_vk(main_vk, false)?;

    for modifier in modifiers.iter().rev() {
        send_vk(*modifier, false)?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn send_vk(vk: VIRTUAL_KEY, press: bool) -> Result<(), String> {
    let scan = unsafe { MapVirtualKeyW(vk.0 as u32, MAPVK_VK_TO_VSC) as u16 };
    if scan == 0 {
        return Err(format!("无法映射 Windows 按键: {}", vk.0));
    }

    let mut flags = KEYEVENTF_SCANCODE;
    if !press {
        flags |= KEYEVENTF_KEYUP;
    }
    if is_extended_windows_key(vk) {
        flags |= KEYEVENTF_EXTENDEDKEY;
    }

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let sent = unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
    if sent == 1 {
        Ok(())
    } else {
        Err("Windows SendInput 发送失败".to_string())
    }
}

#[cfg(target_os = "windows")]
fn windows_virtual_key(key: &Key) -> Result<VIRTUAL_KEY, String> {
    match key {
        Key::Character(ch) if ch.is_ascii_alphabetic() => {
            Ok(VIRTUAL_KEY(ch.to_ascii_uppercase() as u16))
        }
        Key::Character(ch) if ch.is_ascii_digit() => Ok(VIRTUAL_KEY(*ch as u16)),
        Key::Character(ch) => windows_symbol_key(*ch)
            .ok_or_else(|| format!("Windows 暂不支持的符号按键: {}", ch)),
        Key::Control => Ok(VK_LCONTROL),
        Key::Alt => Ok(VK_LMENU),
        Key::Shift => Ok(VK_LSHIFT),
        Key::Meta => Ok(VK_LWIN),
        Key::Escape => Ok(VK_ESCAPE),
        Key::Enter => Ok(VK_RETURN),
        Key::Space => Ok(VK_SPACE),
        Key::Tab => Ok(VK_TAB),
        Key::Backspace => Ok(VK_BACK),
        Key::Delete => Ok(VK_DELETE),
        Key::Insert => Ok(VK_INSERT),
        Key::Home => Ok(VK_HOME),
        Key::End => Ok(VK_END),
        Key::PageUp => Ok(VK_PRIOR),
        Key::PageDown => Ok(VK_NEXT),
        Key::Up => Ok(VK_UP),
        Key::Down => Ok(VK_DOWN),
        Key::Left => Ok(VK_LEFT),
        Key::Right => Ok(VK_RIGHT),
        Key::F(1) => Ok(VK_F1),
        Key::F(2) => Ok(VK_F2),
        Key::F(3) => Ok(VK_F3),
        Key::F(4) => Ok(VK_F4),
        Key::F(5) => Ok(VK_F5),
        Key::F(6) => Ok(VK_F6),
        Key::F(7) => Ok(VK_F7),
        Key::F(8) => Ok(VK_F8),
        Key::F(9) => Ok(VK_F9),
        Key::F(10) => Ok(VK_F10),
        Key::F(11) => Ok(VK_F11),
        Key::F(12) => Ok(VK_F12),
        Key::F(number) => Err(format!("Windows 暂不支持 F{} 功能键", number)),
    }
}

#[cfg(target_os = "windows")]
fn windows_symbol_key(ch: char) -> Option<VIRTUAL_KEY> {
    let vk = match ch {
        ';' | ':' => 0xBA,
        '=' | '+' => 0xBB,
        ',' | '<' => 0xBC,
        '-' | '_' => 0xBD,
        '.' | '>' => 0xBE,
        '/' | '?' => 0xBF,
        '`' | '~' => 0xC0,
        '[' | '{' => 0xDB,
        '\\' | '|' => 0xDC,
        ']' | '}' => 0xDD,
        '\'' | '"' => 0xDE,
        _ => return None,
    };
    Some(VIRTUAL_KEY(vk))
}

#[cfg(target_os = "windows")]
fn is_extended_windows_key(vk: VIRTUAL_KEY) -> bool {
    matches!(
        vk,
        VK_DELETE | VK_INSERT | VK_HOME | VK_END | VK_PRIOR | VK_NEXT | VK_UP | VK_DOWN
            | VK_LEFT | VK_RIGHT
    )
}

#[cfg(target_os = "macos")]
fn simulate_key_combo_macos(shortcut: &ParsedShortcut) -> Result<(), String> {
    if should_use_macos_system_events(shortcut) {
        return simulate_key_combo_macos_system_events(shortcut);
    }

    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "macOS 按键模拟失败，请检查辅助功能权限".to_string())?;
    let modifiers = shortcut
        .modifiers
        .iter()
        .map(macos_key_code)
        .collect::<Result<Vec<_>, _>>()?;
    let modifier_flags = macos_modifier_flags(&shortcut.modifiers);
    let main_flags = modifier_flags | macos_key_extra_flags(&shortcut.main_key);
    let main_key = macos_key_code(&shortcut.main_key)?;

    let mut active_flags = CGEventFlags::CGEventFlagNull;
    for (modifier, key) in shortcut.modifiers.iter().zip(modifiers.iter()) {
        active_flags |= macos_modifier_flag(modifier);
        post_macos_key(&event_source, *key, true, active_flags)?;
    }

    if !modifiers.is_empty() {
        thread::sleep(Duration::from_millis(MACOS_MODIFIER_SETTLE_MS));
    }

    post_macos_key(&event_source, main_key, true, main_flags)?;
    thread::sleep(Duration::from_millis(MACOS_KEY_HOLD_MS));
    post_macos_key(&event_source, main_key, false, main_flags)?;

    active_flags = modifier_flags;
    for (modifier, key) in shortcut.modifiers.iter().zip(modifiers.iter()).rev() {
        active_flags.remove(macos_modifier_flag(modifier));
        post_macos_key(&event_source, *key, false, active_flags)?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn should_use_macos_system_events(shortcut: &ParsedShortcut) -> bool {
    matches!(
        shortcut.main_key,
        Key::Up | Key::Down | Key::Left | Key::Right | Key::Home | Key::End | Key::PageUp
            | Key::PageDown
    )
}

#[cfg(target_os = "macos")]
fn simulate_key_combo_macos_system_events(shortcut: &ParsedShortcut) -> Result<(), String> {
    let main_key = macos_key_code(&shortcut.main_key)?;
    let modifiers = shortcut
        .modifiers
        .iter()
        .map(macos_system_events_modifier)
        .collect::<Vec<_>>();
    let modifier_clause = if modifiers.is_empty() {
        String::new()
    } else {
        format!(" using {{{}}}", modifiers.join(", "))
    };
    let script = format!(
        "tell application \"System Events\" to key code {}{}",
        main_key, modifier_clause
    );
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|err| format!("执行 osascript 失败: {}", err))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            Err("macOS System Events 按键模拟失败，请检查辅助功能权限".to_string())
        } else {
            Err(format!("macOS System Events 按键模拟失败: {}", stderr))
        }
    }
}

#[cfg(target_os = "macos")]
fn macos_system_events_modifier(key: &Key) -> &'static str {
    match key {
        Key::Control => "control down",
        Key::Alt => "option down",
        Key::Shift => "shift down",
        Key::Meta => "command down",
        _ => "",
    }
}

#[cfg(target_os = "macos")]
fn macos_modifier_flags(modifiers: &[Key]) -> CGEventFlags {
    let mut flags = CGEventFlags::CGEventFlagNull;
    for modifier in modifiers {
        flags |= macos_modifier_flag(modifier);
    }
    flags
}

#[cfg(target_os = "macos")]
fn macos_modifier_flag(key: &Key) -> CGEventFlags {
    match key {
        Key::Control => CGEventFlags::CGEventFlagControl,
        Key::Alt => CGEventFlags::CGEventFlagAlternate,
        Key::Shift => CGEventFlags::CGEventFlagShift,
        Key::Meta => CGEventFlags::CGEventFlagCommand,
        _ => CGEventFlags::CGEventFlagNull,
    }
}

#[cfg(target_os = "macos")]
fn macos_key_extra_flags(key: &Key) -> CGEventFlags {
    match key {
        Key::Up
        | Key::Down
        | Key::Left
        | Key::Right
        | Key::Home
        | Key::End
        | Key::PageUp
        | Key::PageDown
        | Key::Delete
        | Key::Insert => CGEventFlags::CGEventFlagNumericPad,
        _ => CGEventFlags::CGEventFlagNull,
    }
}

#[cfg(target_os = "macos")]
fn post_macos_key(
    event_source: &CGEventSource,
    key_code: CGKeyCode,
    press: bool,
    flags: CGEventFlags,
) -> Result<(), String> {
    let event = CGEvent::new_keyboard_event(event_source.clone(), key_code, press)
        .map_err(|_| "macOS 按键事件创建失败，请检查辅助功能权限".to_string())?;
    event.set_flags(flags);
    event.post(CGEventTapLocation::HID);
    Ok(())
}

#[cfg(target_os = "macos")]
fn macos_key_code(key: &Key) -> Result<CGKeyCode, String> {
    let code = match key {
        Key::Character(ch) => macos_character_key(*ch)
            .ok_or_else(|| format!("macOS 暂不支持的字符按键: {}", ch))?,
        Key::Control => 59,
        Key::Alt => 58,
        Key::Shift => 56,
        Key::Meta => 55,
        Key::Enter => 36,
        Key::Space => 49,
        Key::Escape => 53,
        Key::Tab => 48,
        Key::Backspace => 51,
        Key::Delete => 117,
        Key::Insert => 114,
        Key::Home => 115,
        Key::End => 119,
        Key::PageUp => 116,
        Key::PageDown => 121,
        Key::Up => 126,
        Key::Down => 125,
        Key::Left => 123,
        Key::Right => 124,
        Key::F(1) => 122,
        Key::F(2) => 120,
        Key::F(3) => 99,
        Key::F(4) => 118,
        Key::F(5) => 96,
        Key::F(6) => 97,
        Key::F(7) => 98,
        Key::F(8) => 100,
        Key::F(9) => 101,
        Key::F(10) => 109,
        Key::F(11) => 103,
        Key::F(12) => 111,
        Key::F(number) => return Err(format!("macOS 暂不支持 F{} 功能键", number)),
    };
    Ok(code)
}

#[cfg(target_os = "macos")]
fn macos_character_key(ch: char) -> Option<CGKeyCode> {
    match ch.to_ascii_lowercase() {
        'a' => Some(0),
        's' => Some(1),
        'd' => Some(2),
        'f' => Some(3),
        'h' => Some(4),
        'g' => Some(5),
        'z' => Some(6),
        'x' => Some(7),
        'c' => Some(8),
        'v' => Some(9),
        'b' => Some(11),
        'q' => Some(12),
        'w' => Some(13),
        'e' => Some(14),
        'r' => Some(15),
        'y' => Some(16),
        't' => Some(17),
        '1' => Some(18),
        '2' => Some(19),
        '3' => Some(20),
        '4' => Some(21),
        '6' => Some(22),
        '5' => Some(23),
        '=' | '+' => Some(24),
        '9' => Some(25),
        '7' => Some(26),
        '-' | '_' => Some(27),
        '8' => Some(28),
        '0' => Some(29),
        ']' | '}' => Some(30),
        'o' => Some(31),
        'u' => Some(32),
        '[' | '{' => Some(33),
        'i' => Some(34),
        'p' => Some(35),
        'l' => Some(37),
        'j' => Some(38),
        '\'' | '"' => Some(39),
        'k' => Some(40),
        ';' | ':' => Some(41),
        '\\' | '|' => Some(42),
        ',' | '<' => Some(43),
        '/' | '?' => Some(44),
        'n' => Some(45),
        'm' => Some(46),
        '.' | '>' => Some(47),
        '`' | '~' => Some(50),
        _ => None,
    }
}

#[cfg(desktop)]
fn show_main_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    use tauri::Manager;

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            #[cfg(desktop)]
            show_main_window(app);
        }))
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
                use tauri::window::Color;
                use tauri::Manager;

                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_background_color(Some(Color(0, 0, 0, 0)));
                }

                #[cfg(debug_assertions)]
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }

                let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
                let quit_i = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show" => {
                            show_main_window(app);
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
                            show_main_window(app);
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
