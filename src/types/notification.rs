use crate::binds::*;
use crate::types::vigem::Vigem;
use crate::types::target::Target;

pub struct X360Notification {
    pub raw: EVT_VIGEM_X360_NOTIFICATION,
    pub large_motor: u8,
    pub small_motor: u8,
    pub led_number: u8
}

impl X360Notification {
    pub fn from_raw(raw: EVT_VIGEM_X360_NOTIFICATION) -> Self {
        Self{raw, large_motor: raw.LargeMotor, small_motor: raw.SmallMotor, led_number: raw.LedNumber}
    }

    pub fn get_client(&self) -> Result<Vigem , ()> {
        if self.raw.Client.is_null(){
           return Err(());
        } else {
            return Ok(Vigem::from_raw(self.raw.Client));
        }
    }

    pub fn get_target(&self) -> Result<Target , ()> {
        if self.raw.Target.is_null(){
           return Err(());
        } else {
            return Ok(Target::from_raw(self.raw.Target));
        }
    }
}