//! `bruce-hal::Input` from the T-Embed's rotary encoder + push button.
//!
//! Software quadrature decode: each poll reads the A/B lines, a transition table gives
//! the direction, and one [`Button`] event is emitted per detent (4 sub-steps). The push
//! button (active low) emits [`Button::Select`] on the press edge.

use bruce_hal::{Button, Input as BruceInput};
use esp_hal::gpio::Input;

/// Quadrature transition table indexed by `(prev << 2) | current` (2 bits each).
const TABLE: [i8; 16] = [0, -1, 1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 0, 1, -1, 0];

pub struct EncoderInput {
    a: Input<'static>,
    b: Input<'static>,
    key: Input<'static>,
    prev: u8,
    accum: i8,
    key_down: bool,
}

impl EncoderInput {
    pub fn new(a: Input<'static>, b: Input<'static>, key: Input<'static>) -> Self {
        let prev = Self::phase(&a, &b);
        Self {
            a,
            b,
            key,
            prev,
            accum: 0,
            key_down: false,
        }
    }

    fn phase(a: &Input<'static>, b: &Input<'static>) -> u8 {
        (u8::from(a.is_high()) << 1) | u8::from(b.is_high())
    }
}

impl BruceInput for EncoderInput {
    fn poll(&mut self) -> Option<Button> {
        let down = self.key.is_low();
        if down && !self.key_down {
            self.key_down = true;
            return Some(Button::Select);
        }
        if !down {
            self.key_down = false;
        }

        let phase = Self::phase(&self.a, &self.b);
        let step = TABLE[usize::from((self.prev << 2) | phase)];
        self.prev = phase;
        self.accum += step;

        // Direction mapping is provisional until verified on hardware; flip if inverted.
        if self.accum >= 4 {
            self.accum = 0;
            return Some(Button::Down);
        }
        if self.accum <= -4 {
            self.accum = 0;
            return Some(Button::Up);
        }
        None
    }
}
