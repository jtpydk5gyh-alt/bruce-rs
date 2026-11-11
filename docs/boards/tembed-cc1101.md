# LilyGo T-Embed CC1101

First target board for bruce-rs. A popular Bruce board with the sub-GHz radio built in.

- Wiki: <https://wiki.lilygo.cc/products/t-embed-series/t-embed-cc1101/>
- Hardware repo: <https://github.com/Xinyuan-LilyGO/T-Embed-CC1101>

## Specs

| Part | Detail |
|------|--------|
| MCU | ESP32-S3-WROOM-1 (Xtensa LX7, dual-core, 240 MHz) |
| Flash / PSRAM | 16 MB flash, 8 MB PSRAM |
| Display | ST7789V IPS TFT, 1.9", 320 × 170 |
| Input | Rotary encoder with push button (mathertel RotaryEncoder on Arduino) |
| Sub-GHz | CC1101 (300–348 / 387–464 / 779–928 MHz) |
| NFC/RFID | PN532 over I2C, address 0x24, 13.56 MHz |
| IR | Transmitter + receiver |
| Audio | Built-in microphone |
| LED | WS2812 (addressable) |
| Power | 3.7 V 1300 mAh, BQ25896 charger, BQ27220 gauge (charge ≤ 600 mA) |

## Rust target

- Chip is **Xtensa** → target `xtensa-esp32s3-none-elf`.
- Needs the Espressif Rust toolchain (`espup` installs the `esp` channel + Xtensa LLVM).
- Flash/monitor with `espflash` (`cargo run` via the runner in `.cargo/config.toml`).

## HAL mapping (planned)

| bruce-hal trait | Hardware | Driver crate (candidate) |
|-----------------|----------|--------------------------|
| `Display` | ST7789V over SPI | `mipidsi` |
| `Input` | rotary encoder + button | `esp-hal` GPIO + encoder logic in `bruce-core` |
| `Storage` | SD card (SPI) / flash | `embedded-sdmmc` / FAT |
| `Rf` (Phase 3) | CC1101 over SPI | `cc1101-embassy` / `cc1101` (immature — may fork our own) |
| `Nfc` (Phase 3) | PN532 over I2C | `pn532` crate |
| WiFi/BLE (Phase 3) | ESP32-S3 radio | `esp-radio` (deauth needs raw-frame spike) |

## TODO before coding drivers

- [ ] Pull exact GPIO assignments from the board's `pins_arduino.h` / upstream config.
- [ ] Confirm the SPI/I2C bus layout (shared buses, CS pins).
- [ ] Confirm display offset/rotation for the 320 × 170 panel.
