//! This library provides a way to access system information such as CPU load, mounted filesystems,
//! network interfaces, etc.

#![allow(unused)]

extern crate bytesize;
extern crate chrono;
#[cfg_attr(any(target_os = "freebsd", target_os = "openbsd", target_os = "macos"), macro_use)]
extern crate lazy_static;
extern crate libc;
extern crate time;
#[cfg(windows)]
extern crate winapi;
#[cfg(any(target_os = "linux", target_os = "android"))]
#[macro_use]
extern crate nom;

pub mod data;
pub mod platform;

pub use self::data::*;
pub use self::platform::Platform;
pub use self::platform::PlatformImpl as System;
