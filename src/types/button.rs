use crate::raw::*;
use std::ops::Add;

pub trait Reportable {
    type Output;
    fn to_raw(&self) -> Self::Output;
    fn to_xusb(&self) -> Option<&XUSBReport>;
    fn to_ds(&self) -> Option<&DSReport>;
}


bitflags! {
    pub struct XButton: u16 {
        const DpadUp = 1;
        const DpadDown = 2;
        const DpadLeft = 4;
        const DpadRight = 8;
        const Start = 16;
        const Back = 32;
        const LeftThumb = 64;
        const RightThumb = 128;
        const LeftShoulder = 256;
        const RightShoulder = 512;
        const Guide = 1024;
        const A = 4096;
        const B = 8192;
        const X = 16384;
        const Y = 32768;
        const Nothing = 0;
    }
}

impl Add for XButton {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.bits() + rhs.bits()).unwrap()
    }
}

#[derive(Clone, Debug, Copy)]
pub struct XUSBReport {
    pub w_buttons: XButton,
    pub b_left_trigger: u8,
    pub b_right_trigger: u8,
    pub s_thumb_lx: i16,
    pub s_thumb_ly: i16,
    pub s_thumb_rx: i16,
    pub s_thumb_ry: i16,
}

// impl From<_XUSB_REPORT> for XUSBReport {
//     fn from(t: _XUSB_REPORT) -> Self {
//         Self{w_buttons: XButton::n(t.wButtons).unwrap() ,b_left_trigger: t.bLeftTrigger, b_right_trigger: t.bRightTrigger, s_thumb_lx: t.sThumbLX, s_thumb_ly: t.sThumbLY, s_thumb_rx: t.sThumbRX, s_thumb_ry: t.sThumbRY}
//     }
// }

impl Reportable for XUSBReport {
    type Output = XUSB_REPORT;
    fn to_raw(&self) -> XUSB_REPORT {
        let report = XUSB_REPORT {
            wButtons: self.w_buttons.bits(),
            bLeftTrigger: self.b_left_trigger,
            bRightTrigger: self.b_right_trigger,
            sThumbLX: self.s_thumb_lx,
            sThumbLY: self.s_thumb_ly,
            sThumbRX: self.s_thumb_rx,
            sThumbRY: self.s_thumb_ry,
        };
        report
    }

    fn to_ds(&self) -> Option<&DSReport> {
        None
    }

    fn to_xusb(&self) -> Option<&XUSBReport> {
        Some(self)
    }
}

impl Default for XUSBReport {
    fn default() -> Self {
        Self {
            w_buttons: XButton::Nothing,
            b_left_trigger: 0,
            b_right_trigger: 0,
            s_thumb_lx: 0,
            s_thumb_ly: 0,
            s_thumb_rx: 0,
            s_thumb_ry: 0,
        }
    }
}

bitflags! {
    pub struct DS4Button: u16 {
        const ThumbRight = 32768;
        const ThumbLeft = 16384;
        const Options = 8192;
        const Share = 4096; // By the way, I wanna say that I ♥ Life is Strange game series
        const TriggerRight = 2048;
        const TriggerLeft = 1024;
        const ShoulderRight = 512;
        const ShoulderLeft = 256;
        const Triangle = 128;
        const Circle = 64;
        const Cross = 32;
        const Square = 16;
        const Nothing = 0;
    }
}

#[derive(Copy, Clone)]
pub enum SpecialButton {
    PS = 1,
    Touchpad = 2,
    Nothing = 0,
}

#[derive(Copy, Clone)]
pub enum DS4Dpad {
    None = 8,
    Northwest = 7,
    West = 6,
    Southwest = 5,
    South = 4,
    Southeast = 3,
    East = 2,
    Northeast = 1,
    North = 0,
}

pub struct DSReport {
    pub b_thumb_lx: u8,
    pub b_thumb_ly: u8,
    pub b_thumb_rx: u8,
    pub b_thumb_ry: u8,
    pub w_buttons: DS4Button,
    pub b_special: SpecialButton,
    pub b_trigger_l: u8,
    pub b_trigger_r: u8,
}

impl Reportable for DSReport {
    type Output = _DS4_REPORT;
    fn to_raw(&self) -> _DS4_REPORT {
        _DS4_REPORT {
            bThumbLX: self.b_thumb_lx,
            bThumbLY: self.b_thumb_ly,
            bThumbRX: self.b_thumb_rx,
            bThumbRY: self.b_thumb_ry,
            wButtons: self.w_buttons.bits(),
            bSpecial: self.b_special as u8,
            bTriggerL: self.b_trigger_l,
            bTriggerR: self.b_trigger_r,
        }
    }

    fn to_ds(&self) -> Option<&DSReport> {
        Some(self)
    }

    fn to_xusb(&self) -> Option<&XUSBReport> {
        None
    }
}

impl Default for DSReport {
    fn default() -> Self {
        Self {
            b_special: SpecialButton::Nothing,
            b_thumb_lx: 0,
            b_thumb_ly: 0,
            b_thumb_rx: 0,
            b_thumb_ry: 0,
            b_trigger_l: 0,
            b_trigger_r: 0,
            w_buttons: DS4Button::Nothing,
        }
    }
}
