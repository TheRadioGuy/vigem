use crate::raw::*;

/// It's a safe abstraction over `PVIGEM_TARGET`
/// Note: If you use from_raw, dont forget to manually call `target.free()` or you will die
#[derive(Debug)]
pub struct Target {
    pub raw: Box<PVIGEM_TARGET>,
    drop: bool,
}

impl Target {
    /// Make a new Target and allocates it.
    /// `tt` is `TargetType`
    pub fn new(tt: TargetType) -> Self {
        let raw;
        match tt {
            TargetType::Xbox360 => raw = unsafe { vigem_target_x360_alloc() },
            TargetType::DualShock4 => raw = unsafe { vigem_target_ds4_alloc() },
        }

        Self {
            raw: Box::new(raw),
            drop: true,
        }
    }

    /// Make safe abstraction over `PVIGEM_TARGET`, use when you get notification
    pub fn from_raw(target: PVIGEM_TARGET) -> Self {
        Self {
            raw: Box::new(target),
            drop: false,
        }
    }

    pub fn size(&self) -> u32 {
        unsafe { (*(*self.raw)).Size }
    }

    pub fn serial_no(&self) -> u32 {
        unsafe { (*(*self.raw)).SerialNo }
    }

    pub fn state(&self) -> TargetState {
        unsafe { TargetState::new((*(*self.raw)).State) }
    }

    pub fn get_vid(&self) -> u16 {
        unsafe { (*(*self.raw)).VendorId }
    }
    pub fn get_pid(&self) -> u16 {
        unsafe { (*(*self.raw)).ProductId }
    }

    pub fn get_type(&self) -> TargetType {
        unsafe { TargetType::new((*(*self.raw)).Type) }
    }

    pub fn closing_notification_threads(&self) -> bool {
        unsafe { (*(*self.raw)).closingNotificationThreads }
    }

    // ! Userdata can be another type and value
    /// Get target userdata(as for now, it works pretty shitty)
    pub fn user_data<T: Sized>(&self) -> Option<&T> {
        unsafe {
            let data: *mut T = (*(*(self.raw))).NotificationUserData.cast();
            if data.is_null() {
                None
            } else {
                Some(&*data)
            }
        }
    }
 
    pub fn index(&self) -> u32 {
        unsafe {
            let index = vigem_target_get_index(*self.raw);
            return index;
        }
    }

    pub fn is_attached(&self) -> bool {
        unsafe {
            match vigem_target_is_attached(*self.raw) {
                1 => true,
                _ => false,
            }
        }
    }

    pub fn unregister_notification(&self) {
        unsafe {
            match self.get_type() {
                TargetType::Xbox360 => vigem_target_x360_unregister_notification(*self.raw),
                TargetType::DualShock4 => vigem_target_ds4_unregister_notification(*self.raw),
            }
        }
    }

    pub fn set_vid(&self, vid: u16) {
        unsafe {
            vigem_target_set_vid(*self.raw, vid);
        }
    }

    pub fn set_pid(&self, pid: u16) {
        unsafe {
            vigem_target_set_pid(*self.raw, pid);
        }
    }

    pub fn free(&mut self) {
        unsafe {
            vigem_target_free(*self.raw);
        }
    }
}

impl Drop for Target {
    /// Always drop a target - we are good boys
    fn drop(&mut self) {
        if self.drop {
            self.free();
        }
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
