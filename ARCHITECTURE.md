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
|        bruce-hal          |  traits: Display, Keyboard, Radio, Nfc,
|  (traits only, no deps)   |  Storage, Rf ... the contract
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

> This list needs to be checked against upstream's current board matrix before Phase 1.
> RISC-V boards are the pragmatic place to start: the Rust Xtensa toolchain is a fork and
> adds friction that RISC-V doesn't.
