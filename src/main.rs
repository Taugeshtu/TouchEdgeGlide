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

mod zone;
use crate::zone::GlideZone;

fn main() {
    println!("TouchEdgeGlide: starting");
    
    let generic_start = 0.15;
    let generic_end = 0.05;
    let generic_speed = 5.0;
    
    let zone_left = GlideZone {
        glide_direction: Vec2 { x:-1.0, y: 0.0 },
        edge_start: generic_start,
        edge_end: generic_end,
        glide_speed: generic_speed
    };
    let zone_right = GlideZone {
        glide_direction: Vec2 { x: 1.0, y: 0.0 },
        edge_start: 1.0 - generic_start,
        edge_end: 1.0 - generic_end,
        glide_speed: generic_speed
    };
    let zone_up = GlideZone {
        glide_direction: Vec2 { x: 0.0, y:-1.0 },
        edge_start: generic_start,
        edge_end: generic_end,
        glide_speed: generic_speed
    };
    let zone_down = GlideZone {
        glide_direction: Vec2 { x: 0.0, y: 1.0 },
        edge_start: 1.0 - generic_start,
        edge_end: 1.0 - generic_end,
        glide_speed: generic_speed
    };
    let zones = [zone_left, zone_right, zone_up, zone_down];
    
    let mut touchpad = match init::find_touchpad() {
        Ok(touchpad) => touchpad,
        Err(e) => panic!("No touchpad found: {}", e),
    };
    println!("TouchEdgeGlide: touchpad device secured! range: x({}..{}) y({}..{})",
        touchpad.min.x, touchpad.max.x,
        touchpad.min.y, touchpad.max.y
    );
    
    let mut output = uinput::default().unwrap()
        .name("TouchEdgeGlide").unwrap()
        .event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Relative(Position(X))).unwrap()
        .event(Relative(Position(Y))).unwrap()
        .create().unwrap();
    println!("TouchEdgeGlide: virtual mouse output established!");
    
    
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
            
            let mut glide = Vec2::ZERO;
            for zone in &zones {
                glide += zone.compute_glide(normalized);
            }
            
            let int_glide = glide.as_ivec2();
            if( int_glide.x != 0 || int_glide.y != 0 ) {
                output.send(X, glide.x as i32).unwrap();
                output.send(Y, glide.y as i32).unwrap();
                output.synchronize().unwrap();
                
                println!("edging at: {}, norm: {}", abs, normalized);
            }
        }
        
        thread::sleep(Duration::from_millis(16));
    }
}