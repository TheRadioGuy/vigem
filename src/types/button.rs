use crate::binds::*;
#[derive(Clone, Debug)]
pub enum XButton {
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    Start,
    Back,
    LeftThumb,
    RightThumb,
    LeftShoulder,
    RightShoulder,
    Guide,
    A,
    B,
    X,
    Y,
    Nothing,
}

impl XButton {
    pub fn to_raw(&self) -> u16 {
        use XButton::*;
        match self {
            DpadUp => 1,
            DpadDown => 2,
            DpadLeft => 4,
            DpadRight => 8,
            Start => 16,
            Back => 32,
            LeftThumb => 64,
            RightThumb => 128,
            LeftShoulder => 256,
            RightShoulder => 512,
            Guide => 1024,
            A => 4096,
            B => 8192,
            X => 16384,
            Y => 32768,
            Nothing => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct XUSBReport {
    pub w_buttons: XButton,
    pub b_left_trigger: u8,
    pub b_right_trigger: u8,
    pub s_thumb_lx: i16,
    pub s_thumb_ly: i16,
    pub s_thumb_rx: i16,
    pub s_thumb_ry: i16,
}

impl XUSBReport {
    pub fn to_raw(&self) -> XUSB_REPORT {
        let buttons = self.w_buttons.to_raw();
        let report = XUSB_REPORT {
            wButtons: buttons,
            bLeftTrigger: self.b_left_trigger,
            bRightTrigger: self.b_right_trigger,
            sThumbLX: self.s_thumb_lx,
            sThumbLY: self.s_thumb_ly,
            sThumbRX: self.s_thumb_rx,
            sThumbRY: self.s_thumb_ry,
        };
        report
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

pub enum DS4Button {
    ThumbRight,
    ThumbLeft,
    Options,
    Share, // By the way, I wanna say that I ♥ Life is Strange game series
    TriggerRight,
    TriggerLeft,
    ShoulderRight,
    ShoulderLeft,
    Triangle,
    Circle,
    Cross,
    Square,
    Nothing,
}

impl DS4Button {
    pub fn to_raw(&self) -> u16 {
        use DS4Button::*;
        match self {
            ThumbRight => 32768,
            ThumbLeft => 16384,
            Options => 8192,
            Share => 4096,
            TriggerRight => 2048,
            TriggerLeft => 1024,
            ShoulderRight => 512,
            ShoulderLeft => 256,
            Triangle => 128,
            Circle => 64,
            Cross => 32,
            Square => 16,
            Nothing => 0,
        }
    }
}

pub enum SpecialButton {
    PS,
    Touchpad,
    Nothing,
}

impl SpecialButton {
    pub fn to_raw(&self) -> u8 {
        use SpecialButton::*;
        match self {
            PS => 1,
            Touchpad => 2,
            Nothing => 0,
        }
    }
}

pub enum DS4Dpad {
    None,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
    East,
    Northeast,
    North,
}

impl DS4Dpad {
    pub fn to_raw(&self) -> u16 {
        use DS4Dpad::*;
        match self {
            None => 8,
            Northwest => 7,
            West => 6,
            Southwest => 5,
            South => 4,
            Southeast => 3,
            East => 2,
            Northeast => 1,
            North => 0,
        }
    }
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

impl DSReport {
    pub fn to_raw(&self) -> _DS4_REPORT {
        _DS4_REPORT {
            bThumbLX: self.b_thumb_lx,
            bThumbLY: self.b_thumb_ly,
            bThumbRX: self.b_thumb_rx,
            bThumbRY: self.b_thumb_ry,
            wButtons: self.w_buttons.to_raw(),
            bSpecial: self.b_special.to_raw(),
            bTriggerL: self.b_trigger_l,
            bTriggerR: self.b_trigger_r,
        }
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
