use std::thread;
use std::time::Duration;

mod init;
use evdev::{EventType, AbsoluteAxisCode};

use crate::init::TouchpadInfo;

fn main() {
    println!("Hello, world!");
    
    let mut touchpad = match init::find_touchpad() {
        Ok(touchpad) => touchpad,
        Err(e) => panic!("No touchpad found: {}", e),
    };
    
    println!("Touchpad range x: ({}..{}) y: ({}..{})", 
    touchpad.x_min, touchpad.x_max, 
    touchpad.y_min, touchpad.y_max);
    
    loop {
        let abs = match touchpad.device.get_abs_state() {
            Ok(abs) => abs,
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                std::io::ErrorKind::NotFound |
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Touchpad gone or inaccessible: {}", e);
                    break;
                }
                _ => {
                    eprintln!("Unexpected error: {}", e);
                    break;
                }
            }
        };
        
        let keystate = match touchpad.device.get_key_state() {
            Ok(keystate) => keystate,
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                std::io::ErrorKind::NotFound |
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Touchpad gone or inaccessible: {}", e);
                    break;
                }
                _ => {
                    eprintln!("Unexpected error: {}", e);
                    break;
                }
            }
        };
        
        if( keystate.contains(evdev::KeyCode::BTN_TOUCH) ) {
        
        let x = abs[AbsoluteAxisCode::ABS_X.0 as usize].value;
        let y = abs[AbsoluteAxisCode::ABS_Y.0 as usize].value;
        println!("x: {} | y: {}", x, y);
        }
        
        thread::sleep(Duration::from_millis(16));
    }
}