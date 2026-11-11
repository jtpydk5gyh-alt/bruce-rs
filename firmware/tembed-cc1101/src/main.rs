//! Firmware entry for the LilyGo T-Embed CC1101 (ESP32-S3).
//!
//! Phase 1: bring up the chip, init the ST7789 display, read the rotary encoder and let
//! it drive the bruce-core menu. Radios come next. Pins from LilyGo's `utilities.h`.

#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod display;
mod encoder;

use alloc::boxed::Box;

use bruce_core::{MenuEvent, MenuNode, Navigator};
use bruce_hal::Input as _;
use display::ScreenUi;
use encoder::EncoderInput;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::Rate;
use esp_println::println;
use mipidsi::models::ST7789;
use mipidsi::options::ColorInversion;
use mipidsi::{interface::SpiInterface, Builder};

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let p = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    println!("bruce-rs booting on LilyGo T-Embed CC1101 (ESP32-S3)");

    // Backlight on (active high on GPIO21).
    let _backlight = Output::new(p.GPIO21, Level::High, OutputConfig::default());

    let cs = Output::new(p.GPIO41, Level::High, OutputConfig::default());
    let dc = Output::new(p.GPIO16, Level::Low, OutputConfig::default());

    let spi = Spi::new(
        p.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(40))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(p.GPIO11)
    .with_mosi(p.GPIO9);

    let spi_device = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    let buffer: &'static mut [u8] = Box::leak(vec![0u8; 512].into_boxed_slice());
    let di = SpiInterface::new(spi_device, dc, buffer);

    let mut delay = Delay::new();
    let panel = Builder::new(ST7789, di)
        .display_size(170, 320)
        .display_offset(35, 0)
        .invert_colors(ColorInversion::Inverted)
        .init(&mut delay)
        .unwrap();
    let mut screen = ScreenUi::new(panel);

    // Rotary encoder: A=4, B=5, button=0, all active-low with pull-ups.
    let mut input = EncoderInput::new(
        Input::new(p.GPIO4, InputConfig::default().with_pull(Pull::Up)),
        Input::new(p.GPIO5, InputConfig::default().with_pull(Pull::Up)),
        Input::new(p.GPIO0, InputConfig::default().with_pull(Pull::Up)),
    );

    let mut nav = Navigator::new(vec![
        MenuNode::submenu("WiFi", vec![MenuNode::leaf("Scan"), MenuNode::leaf("Evil Portal")]),
        MenuNode::submenu("BLE", vec![MenuNode::leaf("Scan"), MenuNode::leaf("Spam")]),
        MenuNode::submenu("RF", vec![MenuNode::leaf("Read"), MenuNode::leaf("Replay")]),
        MenuNode::leaf("NFC"),
        MenuNode::leaf("IR"),
    ]);
    nav.render(&mut screen);
    println!("menu drawn, {} items", nav.items().len());

    loop {
        if let Some(button) = input.poll() {
            if nav.on_button(button) != MenuEvent::None {
                nav.render(&mut screen);
            }
        }
        delay.delay_millis(2);
    }
}
