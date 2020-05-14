#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::binds::{LPVOID, PVIGEM_CLIENT, PVIGEM_TARGET, UCHAR};
use vigem::notification::*;
use vigem::*;

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
        .x360_register_notification::<i32>(&target, Some(handle), 123123123)
        .unwrap();

    // loop {
    //     let mut report = XUSBReport::default();
    //     report.w_buttons = XButton::X;
    //     vigem.x360_update(&target, report).unwrap();
    // }

    let mut report = XUSBReport::default();
    report.w_buttons = XButton::X;
    println!("Send X");
    vigem.x360_update(&target, &report).unwrap();
    std::thread::sleep(std::time::Duration::new(5, 0));
    report.w_buttons = XButton::Y;
    println!("Send Y ");
    vigem.x360_update(&target, &report).unwrap();

    std::thread::sleep(std::time::Duration::new(999999, 0));
}

unsafe extern "C" fn handle(
    client: PVIGEM_CLIENT,
    target: PVIGEM_TARGET,
    large_motor: UCHAR,
    small_motor: UCHAR,
    led_number: UCHAR,
    user_data: LPVOID,
) {
    let notification: X360Notification<i32> = X360Notification::new(
        client,
        target,
        large_motor,
        small_motor,
        led_number,
        user_data,
    );
    let target = notification.get_target();
    println!(
        "Large motor is: {}, small is : {}",
        notification.large_motor, notification.small_motor
    );
    dbg!(target.state());
    dbg!(notification.userdata().unwrap());
}
