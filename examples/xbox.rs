#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::raw::{PVIGEM_CLIENT, PVIGEM_TARGET, UCHAR};
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
    let mut data = 0;

    vigem
        .x360_register_notification(&target, Some(handle), &mut data)
        .unwrap();

    // Now make a XUSBReport. So our controller will press Y button and LT
    let report = XUSBReport {
        w_buttons: XButton::DpadLeft,
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
    user_data: *mut i32,
) {
    // make a safe absraction over all arguments
    let notification = X360Notification::new(
        client,
        target,
        large_motor,
        small_motor,
        led_number,
        user_data,
    );

    // get target and client which we got in our callback
    let mut target = notification.get_target();
    let mut vigem = notification.get_client();

    println!(
        "Large motor is: {}, small is : {}",
        notification.large_motor, notification.small_motor
    );
    dbg!(target.state());
    // Get userdata(I dont know what it is)
    dbg!(notification.userdata());

    // Now we press B button AND DpadDown and RT

    let report = XUSBReport {
        w_buttons: XButton::B | XButton::DpadDown,
        b_right_trigger: 100,
        s_thumb_lx: 32000,
        ..XUSBReport::default()
    };
   target.update(&report).unwrap();
}
