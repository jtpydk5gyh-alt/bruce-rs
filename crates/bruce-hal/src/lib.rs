//! Hardware abstraction for bruce-rs: every board implements these traits,
//! and bruce-core drives them without knowing the concrete hardware.

pub struct Key(pub char);

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

pub trait Keyboard {
    fn poll(&mut self) -> Option<Button>;
    fn read_char(&mut self) -> Option<Key>;
}

pub trait Storage {
    fn read(&mut self, path: &str) -> Option<Vec<u8>>;
    fn write(&mut self, path: &str, data: &[u8]) -> bool;
}

pub trait Board {
    type Display: Display;
    type Keyboard: Keyboard;
    fn display(&mut self) -> &mut Self::Display;
    fn keyboard(&mut self) -> &mut Self::Keyboard;
    fn name(&self) -> &'static str;
}
