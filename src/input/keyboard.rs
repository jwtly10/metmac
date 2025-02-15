use chrono::Utc;
use log::{debug, info, warn};
use rdev::{Event, EventType, Key};

use crate::{models::events::KeyEvent, system::windows};

pub fn handle_keyboard_event(event: &Event) -> Option<KeyEvent> {
    match &event.event_type {
        EventType::KeyPress(_) => {
            debug!("Raw Event: {:?}", event);
            let key_name = parse_name_from_event(event);

            //let application_name: String = match windows::get_focused_window() {
            //    Some(name) => name,
            //    None => {
            //        warn!("Could not get the focused window");
            //        "unknown".to_string()
            //    }
            //};

            //let key_event =
            //    KeyEvent::new(key_name, application_name, Utc::now().timestamp_millis());

            let key_event = KeyEvent::new(
                key_name,
                "unknown".to_string(),
                Utc::now().timestamp_millis(),
            );

            info!("Key Event: {:?}", key_event);
            Some(key_event)
        }
        _ => None,
    }
}

/// Parses the name of the key from the event
fn parse_name_from_event(event: &Event) -> String {
    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                // Modifiers
                Key::MetaLeft => meta_key_name(true),
                Key::MetaRight => meta_key_name(false),
                Key::Alt => alt_key_name(true),
                Key::AltGr => alt_key_name(false),

                Key::ControlLeft => "ctrl_left".to_string(),
                Key::ControlRight => "ctrl_right".to_string(),

                // Special keys
                Key::Backspace => "backspace".to_string(),
                Key::Return => "return".to_string(),
                Key::Delete => "delete".to_string(),
                Key::Escape => "escape".to_string(),
                Key::ShiftLeft => "shift_left".to_string(),
                Key::ShiftRight => "shift_right".to_string(),
                Key::Tab => "tab".to_string(),

                // Arrow keys
                Key::DownArrow => "down".to_string(),
                Key::UpArrow => "up".to_string(),
                Key::LeftArrow => "left".to_string(),
                Key::RightArrow => "right".to_string(),

                // Other keys
                Key::End => "end".to_string(),
                Key::Home => "home".to_string(),
                Key::Insert => "insert".to_string(),
                Key::CapsLock => "caps_lock".to_string(),

                // Function keys
                Key::F1 => "f1".to_string(),
                Key::F2 => "f2".to_string(),
                Key::F3 => "f3".to_string(),
                Key::F4 => "f4".to_string(),
                Key::F5 => "f5".to_string(),
                Key::F6 => "f6".to_string(),
                Key::F7 => "f7".to_string(),
                Key::F8 => "f8".to_string(),
                Key::F9 => "f9".to_string(),
                Key::F10 => "f10".to_string(),
                Key::F11 => "f11".to_string(),
                Key::F12 => "f12".to_string(),

                // Navigation and special keys
                Key::PageDown => "page_down".to_string(),
                Key::PageUp => "page_up".to_string(),
                Key::Space => "space".to_string(),
                Key::PrintScreen => "print_screen".to_string(),
                Key::ScrollLock => "scroll_lock".to_string(),
                Key::Pause => "pause".to_string(),
                Key::NumLock => "num_lock".to_string(),

                // Number row
                Key::BackQuote => "`".to_string(),
                Key::Num1 => "1".to_string(),
                Key::Num2 => "2".to_string(),
                Key::Num3 => "3".to_string(),
                Key::Num4 => "4".to_string(),
                Key::Num5 => "5".to_string(),
                Key::Num6 => "6".to_string(),
                Key::Num7 => "7".to_string(),
                Key::Num8 => "8".to_string(),
                Key::Num9 => "9".to_string(),
                Key::Num0 => "0".to_string(),
                Key::Minus => "-".to_string(),
                Key::Equal => "=".to_string(),

                // Letter keys
                Key::KeyQ => "q".to_string(),
                Key::KeyW => "w".to_string(),
                Key::KeyE => "e".to_string(),
                Key::KeyR => "r".to_string(),
                Key::KeyT => "t".to_string(),
                Key::KeyY => "y".to_string(),
                Key::KeyU => "u".to_string(),
                Key::KeyI => "i".to_string(),
                Key::KeyO => "o".to_string(),
                Key::KeyP => "p".to_string(),
                Key::KeyA => "a".to_string(),
                Key::KeyS => "s".to_string(),
                Key::KeyD => "d".to_string(),
                Key::KeyF => "f".to_string(),
                Key::KeyG => "g".to_string(),
                Key::KeyH => "h".to_string(),
                Key::KeyJ => "j".to_string(),
                Key::KeyK => "k".to_string(),
                Key::KeyL => "l".to_string(),
                Key::KeyZ => "z".to_string(),
                Key::KeyX => "x".to_string(),
                Key::KeyC => "c".to_string(),
                Key::KeyV => "v".to_string(),
                Key::KeyB => "b".to_string(),
                Key::KeyN => "n".to_string(),
                Key::KeyM => "m".to_string(),

                // Punctuation and brackets
                Key::LeftBracket => "[".to_string(),
                Key::RightBracket => "]".to_string(),
                Key::SemiColon => ";".to_string(),
                Key::Quote => "'".to_string(),
                Key::BackSlash => "\\".to_string(),
                Key::IntlBackslash => "\\".to_string(),
                Key::Comma => ",".to_string(),
                Key::Dot => ".".to_string(),
                Key::Slash => "/".to_string(),

                // Numpad
                Key::KpReturn => "numpad_enter".to_string(),
                Key::KpMinus => "numpad_minus".to_string(),
                Key::KpPlus => "numpad_plus".to_string(),
                Key::KpMultiply => "numpad_multiply".to_string(),
                Key::KpDivide => "numpad_divide".to_string(),
                Key::Kp0 => "numpad_0".to_string(),
                Key::Kp1 => "numpad_1".to_string(),
                Key::Kp2 => "numpad_2".to_string(),
                Key::Kp3 => "numpad_3".to_string(),
                Key::Kp4 => "numpad_4".to_string(),
                Key::Kp5 => "numpad_5".to_string(),
                Key::Kp6 => "numpad_6".to_string(),
                Key::Kp7 => "numpad_7".to_string(),
                Key::Kp8 => "numpad_8".to_string(),
                Key::Kp9 => "numpad_9".to_string(),
                Key::KpDelete => "numpad_delete".to_string(),

                // Special
                Key::Function => "fn".to_string(),
                Key::Unknown(code) => match map_custom_unknowns(code) {
                    Some(name) => name,
                    None => {
                        // Try to parse the name as a last resort
                        match &event.name {
                            Some(name) => {
                                // TODO: Should better handle control/unicode chars
                                if name.chars().any(|c| c.is_ascii_punctuation()) {
                                    warn!("unknown key event with special char: {:?}", event);
                                    format!("unknown_{}", code)
                                } else {
                                    name.to_string()
                                }
                            }
                            None => {
                                warn!("unknown key event: {:?}", event);
                                format!("unknown_{}", code)
                            }
                        }
                    }
                },
            }
        }
        _ => panic!("We should not be parsing non-keypress events"),
    }
}

/// Manually maps unknown keys to their corresponding names
fn map_custom_unknowns(code: u32) -> Option<String> {
    match code {
        115 => Some("home".to_string()),
        117 => Some("delete".to_string()),
        62 => Some("ctrl_right".to_string()),
        _ => None,
    }
}

// Some keys have better names for their archs
// These function maps those keys to their better names

#[cfg(target_os = "macos")]
fn meta_key_name(is_left: bool) -> String {
    if is_left {
        "command_left".to_string()
    } else {
        "command_right".to_string()
    }
}

#[cfg(target_os = "macos")]
fn alt_key_name(is_left: bool) -> String {
    if is_left {
        "opt_left".to_string()
    } else {
        "opt_right".to_string()
    }
}
