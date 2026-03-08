use std::thread;
use std::time::Duration;
use glam::{IVec2, Vec2};

mod init;
use crate::init::TouchpadInfo;
use evdev::{EventType, AbsoluteAxisCode};

use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;


fn main() {
    println!("Hello, world!");
    
    let mut touchpad = match init::find_touchpad() {
        Ok(touchpad) => touchpad,
        Err(e) => panic!("No touchpad found: {}", e),
    };
    
    let mut output = uinput::default().unwrap()
        .name("test").unwrap()
        .event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Relative(Position(X))).unwrap()
        .event(Relative(Position(Y))).unwrap()
        .create().unwrap();
    
    println!("Touchpad range x: ({}..{}) y: ({}..{})", 
    touchpad.min.x, touchpad.max.x, 
    touchpad.min.y, touchpad.max.y);
    
    loop {
        let abs_state = match touchpad.device.get_abs_state() {
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
        
        let key_state = match touchpad.device.get_key_state() {
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
        
        if( key_state.contains(evdev::KeyCode::BTN_TOUCH) ) {
            let abs = IVec2 {
                x: abs_state[AbsoluteAxisCode::ABS_X.0 as usize].value,
                y: abs_state[AbsoluteAxisCode::ABS_Y.0 as usize].value };
            let normalized = touchpad.normalise(abs);
            
            let speed = 2.0;
            let mut glide = Vec2::ZERO;
            if( normalized.x < 0.1 ) { glide.x -= speed; }
            if( normalized.x > 0.9 ) { glide.x += speed; }
            if( normalized.y < 0.1 ) { glide.y -= speed; }
            if( normalized.y > 0.9 ) { glide.y += speed; }
            
            output.send(X, glide.x as i32).unwrap();
            output.send(Y, glide.y as i32).unwrap();
            output.synchronize().unwrap();
            
            let int_glide = glide.as_ivec2();
            if( int_glide.x != 0 || int_glide.y != 0 ) {
                println!("edging at: {}, norm: {}", abs, normalized);
            }
        }
        
        thread::sleep(Duration::from_millis(16));
    }
}