//! M5Stack Cardputer (ESP32-S3) board support.
//! Phase 1+ replaces these stubs with the real esp-hal display and keyboard drivers.

#![no_std]

use bruce_hal::{Board, Button, Display, Input};

#[derive(Default)]
pub struct CardputerDisplay;

impl Display for CardputerDisplay {
    fn clear(&mut self) {}
    fn print(&mut self, _x: u16, _y: u16, _text: &str) {}
    fn flush(&mut self) {}
}

#[derive(Default)]
pub struct CardputerInput;

impl Input for CardputerInput {
    fn poll(&mut self) -> Option<Button> {
        None
    }
}

#[derive(Default)]
pub struct Cardputer {
    display: CardputerDisplay,
    input: CardputerInput,
}

impl Board for Cardputer {
    type Display = CardputerDisplay;
    type Input = CardputerInput;

    fn display(&mut self) -> &mut Self::Display {
        &mut self.display
    }
    fn input(&mut self) -> &mut Self::Input {
        &mut self.input
    }
    fn name(&self) -> &'static str {
        "M5Stack Cardputer"
    }
}
