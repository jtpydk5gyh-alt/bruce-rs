//! M5Stack Cardputer (ESP32-S3) board support.
//! Phase 1 replaces these stubs with the real esp-hal display and keyboard drivers.

use bruce_hal::{Board, Button, Display, Key, Keyboard};

#[derive(Default)]
pub struct CardputerDisplay;

impl Display for CardputerDisplay {
    fn clear(&mut self) {}
    fn print(&mut self, _x: u16, _y: u16, _text: &str) {}
    fn flush(&mut self) {}
}

#[derive(Default)]
pub struct CardputerKeyboard;

impl Keyboard for CardputerKeyboard {
    fn poll(&mut self) -> Option<Button> {
        None
    }
    fn read_char(&mut self) -> Option<Key> {
        None
    }
}

#[derive(Default)]
pub struct Cardputer {
    display: CardputerDisplay,
    keyboard: CardputerKeyboard,
}

impl Board for Cardputer {
    type Display = CardputerDisplay;
    type Keyboard = CardputerKeyboard;

    fn display(&mut self) -> &mut Self::Display {
        &mut self.display
    }
    fn keyboard(&mut self) -> &mut Self::Keyboard {
        &mut self.keyboard
    }
    fn name(&self) -> &'static str {
        "M5Stack Cardputer"
    }
}
