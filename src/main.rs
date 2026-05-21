use std::thread;
use std::time::Duration;
use glam::{IVec2, Vec2};

mod device;
mod zone;
mod config;

use crate::zone::{GlideZone, GlideDirection};
use evdev::AbsoluteAxisCode;

use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event::Controller;
use uinput::event::relative::Position;

fn main() {
    let monitor_mode = std::env::args().any(|a| a == "--monitor");
    
    println!( "{}",
        if monitor_mode { "TouchEdgeGlide: starting monitor mode" }
        else { "TouchEdgeGlide: starting" }
    );

    let full_config = config::load_config();
    
    let zones = if let Some(edges) = full_config.edges {
        [
            GlideZone::from_config(GlideDirection::Left, &edges.left),
            GlideZone::from_config(GlideDirection::Right, &edges.right),
            GlideZone::from_config(GlideDirection::Up, &edges.top),
            GlideZone::from_config(GlideDirection::Down, &edges.bottom),
        ]
    } else if let Some(generic) = full_config.generic {
        [
            GlideZone::from_config(GlideDirection::Left, &generic),
            GlideZone::from_config(GlideDirection::Right, &generic),
            GlideZone::from_config(GlideDirection::Up, &generic),
            GlideZone::from_config(GlideDirection::Down, &generic),
        ]
    } else {
        eprintln!("TouchEdgeGlide: Invalid config, no generic or edges section found.");
        std::process::exit(1);
    };

    let update_frequency = if monitor_mode {5.0} else {60.0};
    let sleep_duration_ms = (1000.0 /update_frequency) as u64;
    
    let touchpad = match device::find_touchpad() {
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
        
        if has_touch {
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
                if int_glide.x != 0 || int_glide.y != 0 {
                    if !is_2_touch && !is_3_touch && !is_4_touch && !is_5_touch {
                        let _ = output.as_mut().unwrap().send(Position::X, glide.x as i32);
                        let _ = output.as_mut().unwrap().send(Position::Y, glide.y as i32);
                    }
                    
                    let _ = output.as_mut().unwrap().synchronize();
                }
            }
        }
        
        thread::sleep(Duration::from_millis(sleep_duration_ms));
    }
}
