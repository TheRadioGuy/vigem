use crate::binds::{PVIGEM_CLIENT, PVIGEM_TARGET, VIGEM_ERROR, _VIGEM_TARGET_STATE, _VIGEM_TARGET_TYPE};
use lib::Library;
use crate::types::target::Target;


pub struct Vigem {
    lib: Library,
    vigem: PVIGEM_CLIENT,
}

impl Vigem {
    pub fn new() -> Self {
        let lib = lib::Library::new("VigemClient_x64.dll").unwrap();
        let vigem = Vigem::alloc(&lib);
        Self { lib, vigem }
    }

    fn alloc(lib: &Library) -> PVIGEM_CLIENT {
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

    pub fn target_add(&mut self, target: &Target) -> VIGEM_ERROR {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_CLIENT, PVIGEM_TARGET) -> VIGEM_ERROR> =
                self.lib.get(b"vigem_target_add").unwrap();
            return f(self.vigem, target.raw);
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