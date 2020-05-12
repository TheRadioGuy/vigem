use crate::binds::*;
use crate::types::target::{Target, TargetState};
use crate::types::vigem::Vigem;

#[derive(Debug)]
pub struct X360Notification {
    pub large_motor: u8,
    pub small_motor: u8,
    pub led_number: u8,
}

impl X360Notification {
    pub fn from_raw(raw: *mut EVT_VIGEM_X360_NOTIFICATION) -> Self {
        unsafe {
            let raw = *raw;
            return Self {
                large_motor: raw.LargeMotor,
                small_motor: raw.SmallMotor,
                led_number: raw.LedNumber,
            };
        }
    }
}

pub struct DS4Notification {
    pub large_motor: u8,
    pub small_motor: u8,
    pub light_bar: LIGHTBAR_COLOR,
}

pub struct LIGHTBAR_COLOR {
    red: u8,
    green: u8,
    blue: u8
}

impl LIGHTBAR_COLOR {
    pub fn new(red:u8,green:u8,blue:u8) -> Self {
        Self{red, green, blue}
    }
}

impl DS4Notification {
    pub fn from_raw(raw: *mut EVT_VIGEM_DS4_NOTIFICATION) -> Self {
        unsafe {
            let raw = *raw;
            let light_bar = raw.LightBar;
            return Self {
                large_motor: raw.LargeMotor,
                small_motor: raw.SmallMotor,
                light_bar: LIGHTBAR_COLOR::new(light_bar.Red, light_bar.Green, light_bar.Blue),
            };
        }
    }
}
