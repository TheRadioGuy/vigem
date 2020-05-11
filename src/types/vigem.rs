
use crate::types::target::Target;
use crate::binds::*;

pub struct Vigem {
    vigem: PVIGEM_CLIENT,
}

impl Vigem {
    pub fn new() -> Self {
        let vigem = unsafe{ vigem_alloc()};
        Self { vigem }
    }

    pub fn from_raw(vigem: PVIGEM_CLIENT) -> Self {
        Self { vigem }
    }

    pub fn connect(&mut self) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_connect(self.vigem);
            let err = VigemError::new(err);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn target_add(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_add(self.vigem, target.raw);
            let err = VigemError::new(err);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn target_remove(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_remove(self.vigem, target.raw);
            let err = VigemError::new(err);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn free(&mut self) {
        unsafe {
            vigem_free(self.vigem);
        }
    }

    pub fn disconnect(&mut self) {
        unsafe {
            vigem_disconnect(self.vigem);
        }
    }

    pub fn xbox_get_user_index(&mut self, target: &Target) -> u32 {
        unsafe {
            let mut index = 0u32;
            let mut index_ptr: *mut u32 = &mut index;
            vigem_target_x360_get_user_index(self.vigem, target.raw, index_ptr);
            return index;
        }
    }

    // TODO: Make other  struct for REPORT
    pub fn x360_update(&mut self, target: &Target, report: crate::XUSB_REPORT) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_x360_update(self.vigem, target.raw, report);
            let err = VigemError::new(err);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }

        }
    }

    pub fn x360_register_notification(
        &mut self,
        target: &Target,
        func: unsafe extern "C" fn(EVT_VIGEM_X360_NOTIFICATION),
        data: i32
    ) -> Result<(), VigemError> {
        unsafe {
            let data_ptr = data as *mut i32;
            let err = vigem_target_x360_register_notification(self.vigem, target.raw, Some(func), data_ptr.cast());
            let err = VigemError::new(err);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }
}

impl Drop for Vigem {
    fn drop(&mut self) {
        self.disconnect();
        self.free();
    }
}

#[derive(Debug)]
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
