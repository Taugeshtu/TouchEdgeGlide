use std::thread;
use std::time::Duration;
use glam::{IVec2, Vec2};

mod init;
use crate::init::TouchpadInfo;
use evdev::{EventType, AbsoluteAxisCode};

use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position;

mod zone;
use crate::zone::GlideZone;

fn main() {
    let monitor_mode = std::env::args().any(|a| a == "--monitor");
    
    println!( "{}",
        if monitor_mode { "TouchEdgeGlide: starting monitor mode" }
        else { "TouchEdgeGlide: starting" }
    );
    
    let update_frequency = if monitor_mode {5.0} else {60.0};
    let sleep_duration_ms = (1000.0 /update_frequency) as u64;
    
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
        Err(e) => {
            eprintln!("No touchpad found: {}", e);
            std::process::exit(1);
        }
    };
    println!("TouchEdgeGlide: touchpad device secured! range: x({}..{}) y({}..{})",
        touchpad.min.x, touchpad.max.x,
        touchpad.min.y, touchpad.max.y
    );
    
    let mut output = if !monitor_mode {
        let dev = uinput::default().unwrap()
            .name("TouchEdgeGlide").unwrap()
            .event(Controller(Mouse(Left))).unwrap()
            .event(Position::X).unwrap()
            .event(Position::Y).unwrap()
            .create().unwrap();
        println!("TouchEdgeGlide: virtual mouse output established!");
        Some(dev)
    } else {
        println!("TouchEdgeGlide: monitor mode, no output device");
        None
    };
    
    loop {
        let abs_state = match touchpad.device.get_abs_state() {
            Ok(abs) => abs,
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                std::io::ErrorKind::NotFound |
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Touchpad gone or inaccessible: {}", e);
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Unexpected error: {}", e);
                    std::process::exit(1);
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
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Unexpected error: {}", e);
                    std::process::exit(1);
                }
            }
        };
        
        let has_touch = key_state.contains(evdev::KeyCode::BTN_TOUCH);
        let is_2_touch = key_state.contains(evdev::KeyCode::BTN_TOOL_DOUBLETAP);
        let is_3_touch = key_state.contains(evdev::KeyCode::BTN_TOOL_TRIPLETAP);
        let is_4_touch = key_state.contains(evdev::KeyCode::BTN_TOOL_QUADTAP);
        let is_5_touch = key_state.contains(evdev::KeyCode::BTN_TOOL_QUINTTAP);
        
        if( has_touch ) {
            let abs = IVec2 {
                x: abs_state[AbsoluteAxisCode::ABS_X.0 as usize].value,
                y: abs_state[AbsoluteAxisCode::ABS_Y.0 as usize].value
            };
            let normalized = touchpad.normalise(abs);
            
            if monitor_mode {
                println!("touch: x={:.2}, y={:.2}", normalized.x, normalized.y);
            } else {
                let mut glide = Vec2::ZERO;
                for zone in &zones {
                    glide += zone.compute_glide(normalized);
                }
                
                let int_glide = glide.as_ivec2();
                if( int_glide.x != 0 || int_glide.y != 0 ) {
                    if( !is_2_touch && !is_3_touch && !is_4_touch && !is_5_touch ) {
                        output.as_mut().unwrap().send(Position::X, glide.x as i32);
                        output.as_mut().unwrap().send(Position::Y, glide.y as i32);
                    }
                    
                    output.as_mut().unwrap().synchronize();
                }
            }
        }
        
        thread::sleep(Duration::from_millis(sleep_duration_ms));
    }
}
