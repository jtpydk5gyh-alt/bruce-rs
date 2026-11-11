//! Hardware-agnostic application logic: menus, modules and navigation.
//! Testable on the host because it only talks to bruce-hal traits.
//!
//! `no_std` in firmware builds, `std` under `cargo test` so the test harness links.

#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;

pub mod gps;
pub mod ir;
pub mod subghz;

use alloc::vec::Vec;
use bruce_hal::{Button, Display};

pub struct Menu {
    pub items: Vec<&'static str>,
    pub cursor: usize,
}

impl Menu {
    pub fn new(items: Vec<&'static str>) -> Self {
        Self { items, cursor: 0 }
    }

    pub fn on_button(&mut self, button: Button) {
        match button {
            Button::Up => self.cursor = self.cursor.checked_sub(1).unwrap_or(self.items.len() - 1),
            Button::Down => self.cursor = (self.cursor + 1) % self.items.len(),
            _ => {}
        }
    }

    pub fn render(&self, display: &mut impl Display) {
        display.clear();
        for (i, item) in self.items.iter().enumerate() {
            let prefix = if i == self.cursor { ">" } else { " " };
            display.print(0, i as u16, &format!("{prefix} {item}"));
        }
        display.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_around_both_ends() {
        let mut menu = Menu::new(vec!["WiFi", "BLE", "RF"]);
        menu.on_button(Button::Up);
        assert_eq!(menu.cursor, 2);
        menu.on_button(Button::Down);
        assert_eq!(menu.cursor, 0);
    }
}
