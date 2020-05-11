use crate::binds::{
    LPVOID, PFN_VIGEM_X360_NOTIFICATION, PULONG, PVIGEM_CLIENT, PVIGEM_TARGET, VIGEM_ERROR,
    _VIGEM_TARGET_STATE, _VIGEM_TARGET_TYPE,EVT_VIGEM_X360_NOTIFICATION
};
use crate::types::target::Target;
use lib::Library;

pub struct Vigem {
    lib: Library,
    vigem: PVIGEM_CLIENT,
}

impl Vigem {
    pub fn new() -> Self {
        let lib = lib::Library::new(crate::types::consts::DLL_NAME).unwrap();
        let vigem = Vigem::alloc(&lib);
        Self { lib, vigem }
    }

    pub fn from_raw(vigem: PVIGEM_CLIENT) -> Self {
        let lib = lib::Library::new(crate::types::consts::DLL_NAME).unwrap();
        Self { lib, vigem }
    }

    pub fn alloc(lib: &Library) -> PVIGEM_CLIENT {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn() -> PVIGEM_CLIENT> =
                lib.get(b"vigem_alloc").unwrap();
            return f();
        }
    }

    pub fn connect(&mut self) -> Result<(), VigemError> {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT) -> VIGEM_ERROR> =
                self.lib.get(b"vigem_connect").unwrap();
            let err = VigemError::new(f(self.vigem));
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn target_add(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT, PVIGEM_TARGET) -> VIGEM_ERROR> =
                self.lib.get(b"vigem_target_add").unwrap();
            let r = f(self.vigem, target.raw);
            let err = VigemError::new(r);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn target_remove(&mut self, target: &Target) -> Result<(), VigemError> {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT, PVIGEM_TARGET) -> VIGEM_ERROR> =
                self.lib.get(b"vigem_target_remove").unwrap();
            let r = f(self.vigem, target.raw);
            let err = VigemError::new(r);
            if err.is_err() {
                return Err(err);
            } else {
                return Ok(());
            }
        }
    }

    pub fn free(&mut self) {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT)> =
                self.lib.get(b"vigem_free").unwrap();
            return f(self.vigem);
        }
    }

    pub fn disconnect(&mut self) {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT)> =
                self.lib.get(b"vigem_disconnect").unwrap();
            return f(self.vigem);
        }
    }

    pub fn xbox_get_user_index(&mut self, target: &Target) -> u32 {
        unsafe {
            let mut index = 0u32;
            let mut index_ptr: *mut u32 = &mut index;
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT, PVIGEM_TARGET, PULONG)> =
                self.lib.get(b"vigem_target_x360_get_user_index").unwrap();
            f(self.vigem, target.raw, index_ptr);
            return index;
        }
    }

    pub fn x360_update(&mut self, target: &Target, report: crate::XUSB_REPORT) -> Result<(), VigemError> {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT, PVIGEM_TARGET, crate::XUSB_REPORT) -> VIGEM_ERROR> =
                self.lib.get(b"vigem_target_x360_update").unwrap();
            let err =VigemError::new(f(self.vigem, target.raw, report));
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
    ) -> Result<(), VigemError> {
        unsafe {
            let f: lib::Symbol<
                unsafe extern "C" fn(
                    PVIGEM_CLIENT,
                    PVIGEM_TARGET,
                    PFN_VIGEM_X360_NOTIFICATION,
                    LPVOID,
                ) -> VIGEM_ERROR,
            > = self
                .lib
                .get(b"vigem_target_x360_register_notification")
                .unwrap();

            let data = 12;
            let data_ptr = data as *mut i32;

            let r = f(self.vigem, target.raw, Some(func), data_ptr.cast());
            let err = VigemError::new(r);
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
