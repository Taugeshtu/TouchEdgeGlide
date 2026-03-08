use evdev::{AbsoluteAxisCode, Device};
use std::error::Error;

pub struct TouchpadInfo {
    pub device: Device,
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
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
                    x_min: x.minimum(),
                    x_max: x.maximum(),
                    y_min: y.minimum(),
                    y_max: y.maximum(),
                });
            }
        }
    }
    Err("No touchpad found".into())
}
