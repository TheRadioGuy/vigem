#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
// TODO: Get avoid of unnecessarry crate
#[macro_use]
extern crate bitflags;

pub mod binds;
pub mod types;

pub use binds as raw;
pub use types::button::*;
pub use types::notification;
pub use types::target::*;
pub use types::vigem::*;
