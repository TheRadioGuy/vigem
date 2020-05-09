use lib::Library;

use crate::binds::{
    PVIGEM_CLIENT, PVIGEM_TARGET, VIGEM_ERROR, _VIGEM_TARGET_STATE, _VIGEM_TARGET_TYPE,
};

pub struct Target {
    pub raw: PVIGEM_TARGET,
    lib: Library,
}

impl Target {
    pub fn new(tt: TargetType) -> Self {
        let lib = lib::Library::new("VigemClient_x64.dll").unwrap();
        let mut raw;
        match tt {
            TargetType::Xbox360 => {
                raw = Target::target_x360_alloc(&lib);
            }
            TargetType::DualShock4 => {
                raw = Target::target_ds4_alloc(&lib);
            }
        }

        Self { raw, lib }
    }

    pub fn size(&self) -> u32 {
        unsafe { (*self.raw).Size }
    }

    pub fn serial_no(&self) -> u32 {
        unsafe { (*self.raw).SerialNo }
    }

    pub fn state(&self) -> TargetState {
        unsafe { TargetState::new((*self.raw).State) }
    }

    pub fn get_vid(&self) -> u16 {
        unsafe { (*self.raw).VendorId }
    }
    pub fn get_pid(&self) -> u16 {
        unsafe { (*self.raw).ProductId }
    }

    pub fn get_type(&self) -> TargetType {
        unsafe { TargetType::new((*self.raw).Type) }
    }

    pub fn closing_notification_threads(&self) -> bool {
        unsafe { (*self.raw).closingNotificationThreads }
    }

    // pub fn notification(&self) -> u32 {
    //     unsafe { (*self.raw).Notification }
    // }
    // pub fn notification_user_data(&self) -> std::ffi::c_void {
    //     unsafe { (*self.raw).NotificationUserData }
    // }
    // pub fn cancel_notification_thread_event(&self) -> u32 {
    //     unsafe { (*self.raw).cancelNotificationThreadEvent }
    // }
    // pub fn notification_thread_list(&self) -> u32 {
    //     unsafe { (*self.raw).notificationThreadList }
    // }

    pub fn index(&self) -> u32 {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET) -> u32> =
                self.lib.get(b"vigem_target_get_index").unwrap();
            return f(self.raw);
        }
    }

    pub fn is_attached(&self) -> bool {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET) -> bool> =
                self.lib.get(b"vigem_target_is_attached").unwrap();
            return f(self.raw);
        }
    }

    fn target_x360_alloc(lib: &Library) -> PVIGEM_TARGET {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn() -> PVIGEM_TARGET> =
                lib.get(b"vigem_target_x360_alloc").unwrap();
            return f();
        }
    }

    pub fn unregister_notification(&self){
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET)>;
            match self.get_type() {
                TargetType::Xbox360 => {f = self.lib.get(b"vigem_target_x360_unregister_notification").unwrap()},
                TargetType::DualShock4 => {f = self.lib.get(b"vigem_target_ds4_unregister_notification").unwrap()}
            }
            return f(self.raw);
        }
    }

    pub fn set_vid(&self, vid: u16) {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET, u16)> =
                self.lib.get(b"vigem_target_set_vid").unwrap();
            return f(self.raw, vid);
        }
    }

    pub fn set_pid(&self, pid: u16) {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET, u16)> =
                self.lib.get(b"vigem_target_set_pid").unwrap();
            return f(self.raw, pid);
        }
    }

    fn target_ds4_alloc(lib: &Library) -> PVIGEM_TARGET {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn() -> PVIGEM_TARGET> =
                lib.get(b"vigem_target_ds4_alloc").unwrap();
            return f();
        }
    }

    fn target_free(&mut self) {
        unsafe {
            let f: lib::Symbol<unsafe extern "C" fn(PVIGEM_TARGET)> =
                self.lib.get(b"vigem_target_free").unwrap();
            return f(self.raw);
        }
    }
}

impl Drop for Target {
    /// Always drop a target - we are good boys
    fn drop(&mut self) {
        self.target_free();
    }
}

#[derive(Debug)]
pub enum TargetType {
    Xbox360,
    DualShock4,
}

impl TargetType {
    pub fn new(tt: _VIGEM_TARGET_TYPE) -> Self {
        match tt {
            0 => TargetType::Xbox360,
            2 => TargetType::DualShock4,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum TargetState {
    New,
    Initialized,
    Connected,
    Disconnected,
}

impl TargetState {
    pub fn new(s: _VIGEM_TARGET_STATE) -> Self {
        use TargetState::*;
        match s {
            0 => New,
            1 => Initialized,
            2 => Connected,
            3 => Disconnected,
            _ => unreachable!(),
        }
    }
}
