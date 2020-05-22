#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#[macro_use]
extern crate bitflags;
pub extern crate vigem_sys;

pub mod types;

pub use vigem_sys as raw;
pub use types::button::*;
pub use types::notification;
pub use types::target::*;
pub use types::vigem::*;
