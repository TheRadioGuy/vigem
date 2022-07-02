use crate::raw::*;
use crate::types::target::Target;
use crate::types::vigem::Vigem;

#[derive(Debug)]
pub struct X360Notification<'a, T: Sized> {
    pub large_motor: u8,
    pub small_motor: u8,
    pub led_number: u8,
    user_data: &'a T,
    client: Box<PVIGEM_CLIENT>,
    target: Box<PVIGEM_TARGET>,
}

impl<T: Sized> X360Notification<'_, T> {
    pub fn new(
        client: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        large_motor: UCHAR,
        small_motor: UCHAR,
        led_number: UCHAR,
        user_data: *mut T,
    ) -> Self {
        let user_data = unsafe { &*(user_data.cast()) };
        Self {
            large_motor,
            small_motor,
            led_number,
            user_data,
            client: Box::new(client),
            target: Box::new(target),
        }
    }

    pub fn get_target(&self) -> Target {
        let target = *self.target;
        Target::from_raw(target, *self.client)
    }

    pub fn get_client(&self) -> Vigem {
        Vigem::from_raw(*self.client)
    }

    pub fn userdata(&self) -> &T {
        &self.user_data
    }
}

pub struct DS4Notification<'a, T: Sized> {
    pub large_motor: u8,
    pub small_motor: u8,
    pub light_bar: LIGHTBAR_COLOR,
    user_data: Option<&'a T>,
    client: Box<PVIGEM_CLIENT>,
    target: Box<PVIGEM_TARGET>,
}

#[derive(Debug)]
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

impl<T: Sized> DS4Notification<'_, T> {
    pub fn from_raw(
        client: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        large_motor: UCHAR,
        small_motor: UCHAR,
        light_bar: DS4_LIGHTBAR_COLOR,
        user_data: LPVOID,
    ) -> Self {
        let user_data = unsafe { // TODO: ACCESS_VIOLATION
            if user_data.is_null(){
                None
            } else {
                let casted: *mut T = user_data.cast();
                if casted.is_null(){
                    None
                } else{
                    Some(&*casted)
                }
            }
        };

        Self {
            large_motor,
            small_motor,
            light_bar: LIGHTBAR_COLOR::new(light_bar.Red, light_bar.Green, light_bar.Blue),
            user_data,
            client: Box::new(client),
            target: Box::new(target),
        }
    }

    pub fn get_target(&self) -> Target {
        Target::from_raw(*self.target, *self.client)
    }

    pub fn get_client(&self) -> Vigem {
        Vigem::from_raw(*self.client)
    }

    pub fn userdata(&self) -> &Option<&T> {
        &self.user_data
    }
}
