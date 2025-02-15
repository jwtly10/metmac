use std::any::type_name_of_val;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use objc2_foundation::NSString;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWindowListCopyWindowInfo(option: u32, relative_to: u32) -> CFArrayRef;
}

#[derive(Debug)]
pub struct WindowInfo {
    pub name: Option<String>,
    pub owner: Option<String>,
    pub layer: Option<i32>,
}

impl WindowInfo {}

pub fn get_focused_window() -> Option<String> {
    println!("Starting get_focused_window");

    unsafe {
        let window_list = CGWindowListCopyWindowInfo(2, 0);
        println!("Got window list");

        let array = CFArray::<CFDictionary>::wrap_under_create_rule(window_list);
        println!("Number of windows: {}", array.len());

        let mut ws: Vec<WindowInfo> = Vec::with_capacity(array.len() as usize);

        for window in array.iter() {
            //println!("{:?}", window);
            let s = type_name_of_val(&window);
            //println!("{:?}", s);

            let (keys, values) = window.get_keys_and_values();

            let mut w = WindowInfo {
                name: None,
                owner: None,
                layer: None,
            };

            for i in 0..keys.len() {
                let key_ptr = keys[i];

                let key_ptr = keys[i];
                let k = CFString::wrap_under_get_rule(key_ptr as *const _);
                //println!("Found key: {}", k.to_string());

                if k.to_string() == "kCGWindowLayer" {
                    let value_ptr = values[i];
                    let v = CFNumber::wrap_under_get_rule(value_ptr as *const _);
                    let v = v.to_i32().unwrap();
                    w.layer = Some(v);
                }

                if k.to_string() == "kCGWindowName" {
                    let value_ptr = values[i];
                    let v = CFString::wrap_under_get_rule(value_ptr as *const _);
                    //println!("{}: {}", k.to_string(), v.to_string());
                    w.name = Some(v.to_string());
                }
                if k.to_string() == "kCGWindowOwnerName" {
                    let value_ptr = values[i];
                    let v = CFString::wrap_under_get_rule(value_ptr as *const _);
                    //println!("{}: {}", k.to_string(), v.to_string());
                    w.owner = Some(v.to_string());
                }
            }

            if let Some(l) = w.layer {
                // Seems only focusable apps are on layer 0
                if l == 0 {
                    // Print the raw window
                    println!("{:?}", window);
                }
            }
            ws.push(w);
        }

        println!("Windows:");
        for w in ws.iter() {
            if let Some(l) = w.layer {
                // Seems only focusable apps are on layer 0
                if l == 0 {
                    println!("{:?}", w);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_focused_window() {
        println!("Starting test");
        let window = get_focused_window();
        println!("Test completed, result: {:?}", window);
    }
}
