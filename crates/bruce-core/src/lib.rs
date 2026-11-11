//! Hardware-agnostic application logic: menus, modules and navigation.
//! Testable on the host because it only talks to bruce-hal traits.
//!
//! `no_std` in firmware builds, `std` under `cargo test` so the test harness links.

#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;

pub mod gps;
pub mod ir;
pub mod menu;
pub mod nfc;
pub mod subghz;

pub use menu::{MenuEvent, MenuNode, Navigator};
