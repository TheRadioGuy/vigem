#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::*;
use vigem::notification::*;

pub fn main() {
    let mut vigem = Vigem::new();
    vigem.connect().unwrap();

    let target = Target::new(TargetType::Xbox360);
    dbg!(target.state());
    vigem.target_add(&target).unwrap();
    dbg!(target.state());
    dbg!(vigem.xbox_get_user_index(&target));
    println!("Pointer to target: {:p}", target.raw);
    vigem
        .x360_register_notification(&target, Some(handle), 123)
        .unwrap();
    std::thread::sleep(std::time::Duration::new(999999, 0));
}

unsafe extern "C" fn handle(data: *mut vigem::raw::EVT_VIGEM_X360_NOTIFICATION) {
        println!("still get info");
        println!("Pointer to data: {:p}", data);
        println!("Data itself is: {:?}", *data);
        
        let notification: X360Notification<i32> = X360Notification::from_raw(data);
        println!("Userdata is {:?}", notification.userdata());
 }
