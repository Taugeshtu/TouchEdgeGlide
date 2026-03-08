use evdev::{AbsoluteAxisCode, Device};
use std::error::Error;
use glam::IVec2;
use glam::Vec2;

pub struct TouchpadInfo {
    pub device: Device,
    pub min: IVec2,
    pub max: IVec2
}

impl TouchpadInfo {
    pub fn normalise(&self, abs: IVec2) -> (Vec2) {
        let range = (self.max - self.min).as_vec2();
        return ((abs - self.min).as_vec2()) / range;
    }
}

pub fn find_touchpad() -> Result<TouchpadInfo, Box<dyn Error>> {
    for path in evdev::enumerate().map(|p| p.0) {
        let dev = Device::open(&path)?;
        let props = dev.properties();
        if props.contains(evdev::PropType::POINTER) {
            let supported = dev
                .supported_absolute_axes()
                .map_or(false, |axes| axes.contains(AbsoluteAxisCode::ABS_X) && axes.contains(AbsoluteAxisCode::ABS_Y));
            if supported {
                let mut abs_info = dev.get_absinfo()?;
                let x = abs_info.find(|(code, _)| *code == AbsoluteAxisCode::ABS_X)
                    .ok_or("No ABS_X")?.1;
                let y = abs_info.find(|(code, _)| *code == AbsoluteAxisCode::ABS_Y)
                    .ok_or("No ABS_Y")?.1;
                drop(abs_info);
                
                return Ok(TouchpadInfo {
                    device: dev,
                    min: IVec2 { x: x.minimum(), y: y.minimum() },
                    max: IVec2 { x: x.maximum(), y: y.maximum() },
                });
            }
        }
    }
    Err("No touchpad found".into())
}
