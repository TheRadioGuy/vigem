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
    vigem.target_add(&target);
    dbg!(target.state());
    
}

