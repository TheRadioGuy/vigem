use vigem::raw::{DS4_LIGHTBAR_COLOR, LPVOID, PVIGEM_CLIENT, PVIGEM_TARGET, UCHAR};
use vigem::notification::*;
use vigem::*;

// TODO: Wrong axes

pub fn main() {
    let mut vigem = Vigem::new();
    vigem.connect().unwrap();

    let mut target = Target::new(TargetType::DualShock4);
    vigem.target_add(&mut target).unwrap();

    vigem
        .ds4_register_notification(&target, Some(handle), 100)
        .unwrap();
    let report = DSReport {
        w_buttons: DS4Button::Circle,
        ..DSReport::default()
    };
    target.update(&report).unwrap();

    std::thread::sleep(std::time::Duration::new(99999999999, 0));
}

unsafe extern "C" fn handle(
    arg1: PVIGEM_CLIENT,
    arg2: PVIGEM_TARGET,
    arg3: UCHAR,
    arg4: UCHAR,
    arg5: DS4_LIGHTBAR_COLOR,
    arg6: LPVOID,
) {
    println!("Getting handle");
    let notification: DS4Notification<u32> =
        DS4Notification::from_raw(arg1, arg2, arg3, arg4, arg5, arg6);
    println!(
        "Lightbar color is: {:?}, large motor is: {} and small is {}",
        notification.light_bar, notification.large_motor, notification.small_motor
    );
}
