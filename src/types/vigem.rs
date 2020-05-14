use crate::binds::*;
use crate::types::target::{Target, TargetType};

pub struct Vigem {
    pub vigem: Box<PVIGEM_CLIENT>,
    drop: bool,
}

impl Vigem {
    pub fn new() -> Self {
        let vigem = unsafe { vigem_alloc() };
        Self {
            vigem: Box::new(vigem),
            drop: true,
        }
    }

    pub fn from_raw(vigem: PVIGEM_CLIENT) -> Self {
        Self {
            vigem: Box::new(vigem),
            drop: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_connect(*self.vigem);
            VigemError::new(err).to_result()
        }
    }

    pub fn target_add(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_add(*self.vigem, *target.raw);
            VigemError::new(err).to_result()
        }
    }

    pub fn add_async(
        &mut self,
        target: &Target,
        func: PFN_VIGEM_TARGET_ADD_RESULT,
    ) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_add_async(*self.vigem, *target.raw, func);
            VigemError::new(err).to_result()
        }
    }

    pub fn target_remove(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_remove(*self.vigem, *target.raw);
            VigemError::new(err).to_result()
        }
    }

    pub fn free(&mut self) {
        unsafe {
            vigem_free(*self.vigem);
        }
    }

    pub fn disconnect(&mut self) {
        unsafe {
            vigem_disconnect(*self.vigem);
        }
    }

    pub fn xbox_get_user_index(&mut self, target: &Target) -> u32 {
        unsafe {
            let mut index = 0u32;
            let index_ptr: *mut u32 = &mut index;
            vigem_target_x360_get_user_index(*self.vigem, *target.raw, index_ptr);
            return index;
        }
    }

    pub fn x360_update(
        &mut self,
        target: &Target,
        report: &crate::types::button::XUSBReport,
    ) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_x360_update(*self.vigem, *target.raw, report.to_raw());
            VigemError::new(err).to_result()
        }
    }

    pub fn ds4_update(
        &mut self,
        target: &Target,
        report: &crate::types::button::DSReport,
    ) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_ds4_update(*self.vigem, *target.raw, report.to_raw());
            VigemError::new(err).to_result()
        }
    }

    pub fn clean(&mut self, target: &Target) -> Result<(), VigemError>{
        match target.get_type() {
            TargetType::Xbox360 => self.x360_update(target, &crate::types::button::XUSBReport::default()),
            TargetType::DualShock4 => self.ds4_update(target, &crate::types::button::DSReport::default())
        }
    }


    /// TODO: Add custom user_data
    pub fn x360_register_notification<T: Sized>(
        &mut self,
        target: &Target,
        func: PFN_VIGEM_X360_NOTIFICATION,
        mut data: T,
    ) -> Result<(), VigemError> {
        unsafe {
            let data_ptr: *mut T = &mut data;
            let err = vigem_target_x360_register_notification(
                *self.vigem,
                *target.raw,
                func,
                data_ptr.cast(),
            );
            VigemError::new(err).to_result()
        }
    }

    pub fn ds4_register_notification(
        &mut self,
        target: &Target,
        func: PFN_VIGEM_DS4_NOTIFICATION,
        data: i32,
    ) -> Result<(), VigemError> {
        unsafe {
            let data_ptr = data as *mut i32;
            let err = vigem_target_ds4_register_notification(
                *self.vigem,
                *target.raw,
                func,
                data_ptr.cast(),
            );
            VigemError::new(err).to_result()
        }
    }
}

impl Drop for Vigem {
    fn drop(&mut self) {
        if self.drop {
            self.disconnect();
            self.free();
        }
    }
}

#[derive(Debug, Clone)]
pub enum VigemError {
    None,
    BusNotFound,
    NoFreeSlot,
    InvalidTarget,
    RemovalFailed,
    AlreadyConnected,
    TargetUninitalized,
    NotPluggedIn,
    BusAccessFailed,
    CallbackAlreadyRegistered,
    CallbackNotFound,
    BusAlreadyConnected,
    BusInvalidHandle,
    XusbUserIndexOutOfRange,
    InvalidParameter,
}

impl VigemError {
    pub fn new(code: VIGEM_ERROR) -> VigemError {
        match code {
            536870912 => VigemError::None,
            536870911 => VigemError::BusNotFound,
            536870910 => VigemError::NoFreeSlot,
            536870909 => VigemError::InvalidTarget,
            536870908 => VigemError::RemovalFailed,
            536870907 => VigemError::AlreadyConnected,
            536870906 => VigemError::TargetUninitalized,
            536870905 => VigemError::NotPluggedIn,
            536870903 => VigemError::BusAccessFailed,
            536870896 => VigemError::CallbackAlreadyRegistered,
            536870895 => VigemError::CallbackNotFound,
            536870894 => VigemError::BusAlreadyConnected,
            536870893 => VigemError::BusInvalidHandle,
            536870892 => VigemError::XusbUserIndexOutOfRange,
            536870891 => VigemError::InvalidParameter,
            _ => unreachable!(),
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            VigemError::None => false,
            _ => true,
        }
    }

    pub fn to_result(&self) -> Result<(), VigemError> {
        if self.is_err() {
            Err(self.clone())
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Display for VigemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for VigemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if self.is_err() {
            return Some(self);
        } else {
            return None;
        }
    }
}
