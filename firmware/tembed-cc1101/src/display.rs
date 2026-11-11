//! `bruce-hal::Display` on top of any embedded-graphics draw target.
//!
//! Generic over the draw target so the concrete mipidsi/SPI type only appears once,
//! at construction in `main`. `print` lays text on a character grid.

use bruce_hal::Display as BruceDisplay;
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

const CELL_W: i32 = FONT_10X20.character_size.width as i32;
const CELL_H: i32 = FONT_10X20.character_size.height as i32 + 2;

pub struct ScreenUi<D> {
    target: D,
    style: MonoTextStyle<'static, Rgb565>,
}

impl<D> ScreenUi<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(target: D) -> Self {
        Self {
            target,
            style: MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
        }
    }
}

impl<D> BruceDisplay for ScreenUi<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    fn clear(&mut self) {
        let _ = self.target.clear(Rgb565::BLACK);
    }

    fn print(&mut self, x: u16, y: u16, text: &str) {
        let at = Point::new(x as i32 * CELL_W, y as i32 * CELL_H);
        let _ = Text::with_baseline(text, at, self.style, Baseline::Top).draw(&mut self.target);
    }

    fn flush(&mut self) {}
}
