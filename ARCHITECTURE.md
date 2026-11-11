# Architecture

The design goal is to support every board Bruce supports without duplicating logic.
That forces a strict split between *what the firmware does* and *how a given board does it*.

```
+---------------------------+
|        board-xxx          |  per-board binary: wires esp-hal peripherals
|  (esp-hal, no_std)        |  into the HAL traits, then runs bruce-core
+-------------+-------------+
              | implements
+-------------v-------------+
|        bruce-hal          |  traits: Display, Input, Storage,
|  (traits only, no deps)   |  Rf, Nfc ... the contract
+-------------^-------------+
              | depends on
+-------------+-------------+
|        bruce-core         |  menus, modules, protocols, app state
|  (hardware-agnostic)      |  tested on the host — no board required
+---------------------------+
```

## Rules

1. `bruce-core` never imports `esp-hal`. If it needs hardware, it needs a HAL trait.
   This is what keeps the logic testable in CI without a device.
2. A board crate is thin: construct peripherals, implement the traits, call into core.
3. Shared protocol code (RF/IR/NFC packet formats, file parsers) lives in `bruce-core`
   because it is pure data — the most valuable code to get right and the easiest to test.

## Supported boards (target)

Mirrors upstream Bruce. Grouped by chip because the compile target differs:

- **ESP32-S3 (Xtensa):** M5Cardputer, M5StickC Plus2, LilyGo T-Embed / T-Deck, devkits.
- **ESP32 (Xtensa):** M5StickC Plus, M5Stack Core/Core2, generic ESP32.
- **ESP32-C3 / C5 / C6 (RISC-V):** better Rust toolchain support; good first real targets.

> This list needs to be checked against upstream's current board matrix.
> **First target: LilyGo T-Embed CC1101 (ESP32-S3, Xtensa)** — see
> [docs/boards/tembed-cc1101.md](docs/boards/tembed-cc1101.md). Xtensa needs the
> Espressif Rust toolchain (`espup`); RISC-V boards (C3/C6) come later and need no fork.

## Chosen crates

Decided after surveying the Rust embedded ecosystem (2026-11). We reuse mature drivers and
only write our own where nothing good exists. Pin exact versions in each crate's manifest.

| Area | Crate | Maturity | Notes |
|------|-------|----------|-------|
| HAL / peripherals | `esp-hal` | Approaching 1.0, official | ESP32-S3 Xtensa well supported |
| WiFi / BLE | `esp-radio` (ex `esp-wifi`) | Pre-1.0, official | Behind `unstable`; BLE via TroubLE stack |
| Display | `mipidsi` + `embedded-graphics` | Mature | ST7789V driver; the one we use |
| nRF24 | `rf24-rs` | Active, official org | Best option, alive |
| NFC (PN532) | `pn532` | Usable, low maintenance | Watch embedded-hal 0.2 vs 1.0 |
| Sub-GHz (CC1101) | `cc1101-embassy` / `cc1101` | **Immature** | Likely write/patch our own — see risks |
| IR, BadUSB/HID | — | No ready crate | Our own logic in `bruce-core` |

## Known risks

- **CC1101 / sub-GHz:** Rust drivers are early. Expect to write or fork one. This is the
  T-Embed's headline radio, so it is high-priority *and* high-effort.
- **WiFi deauth:** the Espressif stack blocks sending deauth management frames the normal
  way (same limit Marauder hits). Deauth is a project goal, so Phase 3 WiFi must prototype
  raw 802.11 frame injection early and confirm it works before committing to the feature.
- **esp-radio is pre-1.0:** APIs will shift; track the 1.0 release.
