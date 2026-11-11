//! Hardware abstraction for bruce-rs: every board implements these traits,
//! and bruce-core drives them without knowing the concrete hardware.
//!
//! `no_std` so the same crate links into firmware; `alloc` is provided by the
//! final board binary (e.g. `esp-alloc`).

#![no_std]

extern crate alloc;

use alloc::vec::Vec;

/// A character typed on boards that have a real keyboard.
pub struct Key(pub char);

/// Directional / action events. A board maps its hardware to these: keyboard arrow
/// keys, a rotary encoder's rotation and press, or discrete buttons.
pub enum Button {
    Up,
    Down,
    Select,
    Back,
}

pub trait Display {
    fn clear(&mut self);
    fn print(&mut self, x: u16, y: u16, text: &str);
    fn flush(&mut self);
}

/// Board input. Every board turns its hardware (encoder, keyboard, buttons) into
/// [`Button`] events. Boards with a full keyboard also override [`Input::read_char`].
pub trait Input {
    fn poll(&mut self) -> Option<Button>;

    fn read_char(&mut self) -> Option<Key> {
        None
    }
}

pub trait Storage {
    fn read(&mut self, path: &str) -> Option<Vec<u8>>;
    fn write(&mut self, path: &str, data: &[u8]) -> bool;
}

/// Infrared transceiver. Durations are mark/space lengths in microseconds, starting with
/// a mark. A board implements this over the ESP32 RMT peripheral.
pub trait Ir {
    fn send_raw(&mut self, durations: &[u16]);

    fn receive_raw(&mut self, _buffer: &mut [u16]) -> usize {
        0
    }
}

pub trait Board {
    type Display: Display;
    type Input: Input;

    fn display(&mut self) -> &mut Self::Display;
    fn input(&mut self) -> &mut Self::Input;
    fn name(&self) -> &'static str;
}
