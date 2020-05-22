use crate::raw::*;
use crate::types::target::{Target, TargetType};
// use crate::types::button::Reportable;
use crate::types::button::{DSReport,XUSBReport, Reportable};
pub struct Vigem {
    pub vigem: Box<PVIGEM_CLIENT>,
    drop: bool,
}

impl Vigem {
    /// Create a new Vigem instance and allocates it
    pub fn new() -> Self {
        let vigem = unsafe { vigem_alloc() };
        Self {
            vigem: Box::new(vigem),
            drop: true,
        }
    }

    /// You can build safe `Vigem` abstraction from `PVIGEM_CLIENT`, which you can obtain from notifications
    pub fn from_raw(vigem: PVIGEM_CLIENT) -> Self {
        Self {
            vigem: Box::new(vigem),
            drop: false,
        }
    }
    /// Initializes the driver object and establishes a connection to the emulation bus driver. Returns an error if no compatible bus device has been found.
    pub fn connect(&mut self) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_connect(*self.vigem);
            VigemError::new(err).to_result()
        }
    }

    /// Adds a provided target device to the bus driver, which is equal to a device plug-in event of a physical hardware device. This function blocks until the target device is in full operational mode.
    pub fn target_add(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_add(*self.vigem, *target.raw);
            VigemError::new(err).to_result()
        }
    }

    /// Add the target but async. As it ready callback with `PVIGEM_CLIENT`, `PVIGEM_TARGET`, `PVIGEM_ERORR` will be called
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

    // TODO: Add to target drop
    /// Removes a provided target device from the bus driver, which is equal to a device unplug event of a physical hardware device. The target device object may be reused
    pub fn target_remove(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let err = vigem_target_remove(*self.vigem, *target.raw);
            VigemError::new(err).to_result()
        }
    }

    /// Free Vigem object, dont forget to remove all targets!
    pub fn free(&mut self) {
        unsafe {
            vigem_free(*self.vigem);
        }
    }

    /// Disconnect from the BUS driver
    pub fn disconnect(&mut self) {
        unsafe {
            vigem_disconnect(*self.vigem);
        }
    }

    /// Get xbox index
    pub fn xbox_get_user_index(&mut self, target: &Target) -> u32 {
        unsafe {
            let mut index = 0u32;
            let index_ptr: *mut u32 = &mut index;
            vigem_target_x360_get_user_index(*self.vigem, *target.raw, index_ptr);
            return index;
        }
    }

    /// Send report, report type depends on target type
    /// For DualShock4, type is: `DSReport`
    /// For Xbox, type is `XUSBReport`
    /// **TODO**:  Add guide how to make report
    pub fn update<T: Reportable>(&mut self, target: &Target, report: &T) -> Result<(), VigemError>{
        unsafe{
            let err = match target.get_type() {
                TargetType::Xbox360 => vigem_target_x360_update(*self.vigem, *target.raw, report.to_xusb().unwrap().to_raw()),
                TargetType::DualShock4 => vigem_target_ds4_update(*self.vigem, *target.raw, report.to_ds().unwrap().to_raw())
            };
            VigemError::new(err).to_result()
        }
    }
  
    /// Register notification, callback will be called as controller got force-feedbacked
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

    /// Register notification, callback will be called as controller got force-feedbacked
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

// TODO: Replace codes to ones from vigem bindings
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
