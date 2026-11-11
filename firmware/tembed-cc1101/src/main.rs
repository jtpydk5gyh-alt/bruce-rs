//! Firmware entry for the LilyGo T-Embed CC1101 (ESP32-S3).
//!
//! Phase 1: bring up the chip, init the ST7789 display and draw the bruce-core menu.
//! Encoder input and radios come next. Pins taken from LilyGo's `utilities.h`.

#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod display;

use alloc::boxed::Box;

use bruce_core::Menu;
use display::ScreenUi;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
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
    let menu = Menu::new(vec!["WiFi", "BLE", "RF", "NFC", "IR"]);
    menu.render(&mut screen);
    println!("menu drawn, {} items", menu.items.len());

    loop {
        delay.delay_millis(1000);
    }
}
