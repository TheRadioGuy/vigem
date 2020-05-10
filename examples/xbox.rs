#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::*;

pub fn main() {
    let mut vigem = Vigem::new();
    vigem.connect().unwrap();

    let target = Target::new(TargetType::Xbox360);
    dbg!(target.state());
    vigem.target_add(&target).unwrap();
    dbg!(target.state());
    dbg!(target.get_type());
    dbg!(vigem.xbox_get_user_index(&target));
    vigem.x360_register_notification(&target, handle).unwrap();
    // vigem.x360_update(&target, XUSB_REPORT {})
    std::thread::sleep(std::time::Duration::new(999999, 0));

    
}


unsafe extern "C" fn handle (data: vigem::binds::EVT_VIGEM_X360_NOTIFICATION) {
    dbg!(data);
}