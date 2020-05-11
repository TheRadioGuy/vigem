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
    dbg!(vigem.xbox_get_user_index(&target));
    vigem
        .x360_register_notification(&target, Some(handle), 11)
        .unwrap();
    unsafe { dbg!((*target.raw).State) };
    std::thread::sleep(std::time::Duration::new(999999, 0));
}

unsafe extern "C" fn handle(data: vigem::raw::EVT_VIGEM_X360_NOTIFICATION) {
        println!("still get info");
        let notification = notification::X360Notification::from_raw(data);
        dbg!(notification.large_motor);
        let target = notification.get_target();
        if target.is_ok() {
            dbg!(target.unwrap().state());
        }

    }
