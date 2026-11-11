//! Firmware entry for the LilyGo T-Embed CC1101 (ESP32-S3).
//!
//! Phase 1 bootstrap: bring up the chip, prove the `no_std` bruce-core logic links and
//! runs on target, and print over serial. Display, encoder and radios come next.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;

use bruce_core::Menu;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    println!("bruce-rs booting on LilyGo T-Embed CC1101 (ESP32-S3)");

    let menu = Menu::new(vec!["WiFi", "BLE", "RF", "NFC", "IR"]);
    println!("bruce-core linked, menu has {} items:", menu.items.len());
    for item in &menu.items {
        println!("  - {item}");
    }

    let delay = Delay::new();
    loop {
        delay.delay_millis(1000);
    }
}
