use std::thread;
use std::time::Duration;

mod init;
use evdev::EventType;

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
        // let keys = touchpad.device.get_key_state()?;
        // let finger_down = keys.contains(KeyCode::BTN_TOUCH);
        
        let events = match touchpad.device.fetch_events() {
            Ok(events) => events,
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
        
        let mut x = 0;
        let mut y = 0;
        
        for event in events {
            if event.event_type() != EventType::ABSOLUTE {
                continue;
            }
            
            if( event.code() == 0 ) {x = event.value();}
            if( event.code() == 1 ) {y = event.value();}
            
            // println!("code: {} | data: {}", event.code(), event.value());
        }
        
        // if( finger_down ) {
            println!("x: {} | y: {}", x, y);
        // }
    }
}