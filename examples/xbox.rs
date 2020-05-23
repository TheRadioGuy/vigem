#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::raw::{LPVOID, PVIGEM_CLIENT, PVIGEM_TARGET, UCHAR};
use vigem::notification::*;
use vigem::*;

pub fn main() {
    // Make a vigem object, which alloc immediantely
    let mut vigem = Vigem::new();
    // connect our client to a VigemBus
    vigem.connect().unwrap();
    // Make a new target which represent XBOX360 controller
    let mut target = Target::new(TargetType::Xbox360);
    // Get controller state - as target isnt connected state is "Initialized"
    dbg!(target.state());
    // Add target to VigemBUS
    vigem.target_add(&mut target).unwrap();
    // Now it's connected!
    dbg!(target.state());

    // It's a bit harder. We register notification. Handle will be called every time controller get forcefeedbacked
    vigem
        .x360_register_notification::<i32>(&target, Some(handle), 123123123)
        .unwrap();

    // Now make a XUSBReport. So our controller will press Y button and LT
    let report = XUSBReport {
        w_buttons: XButton::Guide,
        b_left_trigger: 100,
        ..XUSBReport::default()
    };
    vigem.update(&target, &report).unwrap();

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
    // make a safe absraction over all arguments
    let notification: X360Notification<i32> = X360Notification::new(
        client,
        target,
        large_motor,
        small_motor,
        led_number,
        user_data,
    );

    // get target and client which we got in our callback
    let target = notification.get_target();
    let mut vigem = notification.get_client();

    println!(
        "Large motor is: {}, small is : {}",
        notification.large_motor, notification.small_motor
    );
    dbg!(target.state());
    // Got userdata(I dont know what it is)
    dbg!(notification.userdata());

    // Now we press B button and RT

    let report = XUSBReport {
        w_buttons: XButton::B,
        b_right_trigger: 100,
        s_thumb_lx: 20000,
        ..XUSBReport::default()
    };
    vigem.update(&target, &report).unwrap();
}
