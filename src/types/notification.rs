use crate::binds::*;
use crate::types::target::{Target, TargetState};
use crate::types::vigem::Vigem;

#[derive(Debug)]
pub struct X360Notification<T: Sized> {
    pub large_motor: u8,
    pub small_motor: u8,
    pub led_number: u8,
    user_data: *mut T,
    client: Box<PVIGEM_CLIENT>,
    target: Box<PVIGEM_TARGET>,
}

impl<T: Sized> X360Notification<T> {
    pub fn new(
        client: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        large_motor: UCHAR,
        small_motor: UCHAR,
        led_number: UCHAR,
        user_data: LPVOID,
    ) -> Self {
        unsafe {
            let user_data: *mut T = user_data.cast();
            return Self {
                large_motor,
                small_motor,
                led_number,
                user_data,
                client: Box::new(client),
                target: Box::new(target),
            };
        }
    }

    pub fn get_target(&self) -> Target {
        let target = *self.target;
        Target::from_raw(target)
    }

    pub fn get_client(&self) -> Vigem {
        let client = *self.client;
        let client = Vigem::from_raw(client);
        client
    }

    pub fn userdata(&self) -> Option<&T> {
        unsafe {
            if !self.user_data.is_null() {
                return Some(&*self.user_data);
            } else {
                return None;
            }
        }
    }
}

pub struct DS4Notification<T: Sized> {
    pub large_motor: u8,
    pub small_motor: u8,
    pub light_bar: LIGHTBAR_COLOR,
    user_data: *mut T,
    client: Box<PVIGEM_CLIENT>,
    target: Box<PVIGEM_TARGET>,
}

pub struct LIGHTBAR_COLOR {
    red: u8,
    green: u8,
    blue: u8,
}

impl LIGHTBAR_COLOR {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl<T: Sized> DS4Notification<T> {
    pub fn from_raw(
        client: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        large_motor: UCHAR,
        small_motor: UCHAR,
        light_bar: DS4_LIGHTBAR_COLOR,
        user_data: LPVOID,
    ) -> Self {
        unsafe {
            let user_data: *mut T = user_data.cast();
            return Self {
                large_motor,
                small_motor,
                light_bar: LIGHTBAR_COLOR::new(light_bar.Red, light_bar.Green, light_bar.Blue),
                user_data,
                client: Box::new(client),
                target: Box::new(target),
            };
        }
    }

    pub fn get_target(&self) -> Target {
        let target = *self.target;
        Target::from_raw(target)
    }

    pub fn get_client(&self) -> Vigem {
        let client = *self.client;
        let client = Vigem::from_raw(client);
        client
    }

    pub fn userdata(&self) -> Option<&T> {
        unsafe {
            if !self.user_data.is_null() {
                return Some(&*self.user_data);
            } else {
                return None;
            }
        }
    }
}
