# Roadmap

Bruce is **~410 source files in `src/`** across 16 feature modules plus core
infrastructure — not a small program. Rewriting it in Rust is a multi-month, many-PR
effort. The strategy: grow from a tested, hardware-agnostic core, port **one module per
PR**, and never merge code that fails CI.

Tracked against upstream `BruceDevices/firmware` (pin a specific commit before Phase 3).

**Legend:** ☐ todo · ☑ done · 🔌 needs physical hardware to verify.

---

## Phase 0 — Foundations ☑

- ☑ Cargo workspace: `bruce-hal`, `bruce-core`, `boards/*`.
- ☑ Trait-based HAL (`Display`, `Keyboard`, `Storage`, `Board`).
- ☑ Host-tested core (menu navigation + unit test).
- ☑ CI: `fmt` + `clippy -D warnings` + `test`.
- ☑ `CONTRIBUTING.md`, PR template, `CODEOWNERS`, issue templates.
- ☑ Branch protection on `main` (PR-only, enforced for admins).

---

## Phase 1 — First real target

First board: **LilyGo T-Embed CC1101 (ESP32-S3)**.

- ☑ Pick first chip (ESP32-S3, Xtensa).
- ☑ `esp-hal`, `esp-backtrace`, `esp-println`, `.cargo/config.toml`, `espflash` runner.
- ☑ CI job: cross-compile firmware for the target.
- ☑ `tembed-cc1101` binary that boots and prints over serial.
- ☑ Real `Display` driver (`mipidsi`/ST7789) draws the bruce-core menu.
- ☑ Real `Input` driver (rotary encoder) moves the menu cursor.
- 🔌 ☐ Flash and navigate the menu on device (install `espflash`). **Milestone 1.**

---

## Phase 2 — Core infrastructure

Mirrors Bruce's `src/core/` (connect, menu_items, serial_commands, tftLogger, USBSerial).

- ☐ `Storage` impl: SD (SPI) + internal flash (FAT/SPIFFS).
- ☐ Settings: load/save/defaults, persistence across reboots.
- ☐ Boot flow, main menu, module dispatch matching Bruce's UX.
- ☐ File browser (pure logic over `Storage`, host-tested).
- ☐ Serial command interface + USB serial.
- ☐ `tftLogger` equivalent, panic handler, diagnostics.
- ☐ Async runtime decision: `embassy` vs bare `esp-hal` loop (document it).
- ☐ Theming / UI layer.

---

## Phase 3 — Feature modules

Full inventory from upstream `src/modules/`, with file counts as a scale hint. Ordered
roughly easiest→hardest so pure-logic parsers land and get tested before radios appear.
Each module is its own PR (often several): logic in `bruce-core` with tests, a `bruce-hal`
trait, a driver, then on-device verification.

| Order | Module | Upstream files | What it covers | Notes |
|------:|--------|---------------:|----------------|-------|
| 3.1 | **IR** | 11 | TV-B-Gone, send/receive, NEC/RC5/Sony/Samsung | **Started**: `Ir` trait + NEC encode/decode, host-tested. Next: RC5/Sony + RMT driver |
| 3.2 | **Others: iButton / U2F / QR / timer / clicker / audio / mic** | 18 | Grab-bag of small features | Pick off one by one between big modules |
| 3.3 | **RFID / NFC** | 28 | PN532 / RC522, read/write/emulate, dumps | **Started**: `.nfc` dump parse/write, host-tested. Next: PN532 driver |
| 3.4 | **RF / Sub-GHz** | 40 | CC1101, record/replay, scan, `.sub` files | **Started**: `.sub` parse/write, host-tested. Next: CC1101 driver, scan |
| 3.5 | **NRF24** | 11 | 2.4GHz tooling, jammer, scanner | |
| 3.6 | **GPS** | 8 | NMEA parsing, wardriving logs | **Started**: GGA parser + checksum, host-tested. Next: RMC, more sentences |
| 3.7 | **FM** | 2 | FM radio control | Small |
| 3.8 | **LoRa** | 2 | LoRa send/receive | Small |
| 3.9 | **WiFi** | 29 | scan, evil portal, beacon spam, **deauth**, sniff, wardriving | `esp-radio`; big; see deauth note |
| 3.10 | **Ethernet** | 16 | wired networking features | |
| 3.11 | **BLE** + **BLE API** + **badUSB BLE** | 15+8+2 | scan, spam, HID over BLE | |
| 3.12 | **Pwnagotchi** | 10 | pwnagotchi detection/interaction | Depends on WiFi |
| 3.13 | **Reverse shell** | 2 | remote shell over network | Depends on WiFi/Ethernet |
| 3.14 | **bjs_interpreter** | **55** | **JavaScript/Bruce-script interpreter** | Biggest single piece — see below |

### On WiFi deauth (3.9)

Deauth is a project goal. The Espressif WiFi stack blocks sending deauth management frames
the normal way (the same wall ESP32 Marauder hits). Before building the deauth feature,
the first WiFi task is a spike: prototype **raw 802.11 frame injection** via `esp-radio`
and confirm a deauth frame actually goes out on hardware. If it can't, we document the
limit and ship the rest of the WiFi tooling without it rather than fake it.

### On the script interpreter (3.14)

Bruce embeds a JS interpreter so users run `.js` scripts on the device. This is the
single largest and riskiest port. Options, to decide when we get there:
- Reuse an existing Rust `no_std` JS/script engine if one fits the memory budget.
- Port a smaller scripting language instead.
- Build it last, or treat it as a stretch goal.

Decide with real numbers (flash/RAM headroom) rather than up front.

---

## Phase 4 — Boards & breadth

- ☐ Bring up the first **Xtensa** board (ESP32-S3, e.g. Cardputer) — proves the HAL
  abstraction and the `esp-rs` toolchain in CI.
- ☐ More boards from Bruce's matrix; one `boards/*` crate each.
- ☐ Board-support table in README, pinned to an upstream commit.

---

## Phase 5 — Release & parity

- ☐ Living **feature-parity table** vs upstream (per module: missing / WIP / done / 🔌 verified).
- ☐ CI release pipeline: signed `.bin` per board on tag.
- ☐ Web/USB flasher page or documented `espflash` guide.
- ☐ Reproducible builds.

---

## Cross-cutting tracks

- **Testing:** every bit of logic ships with unit tests; capture fixtures for protocols;
  `clippy -D warnings` stays green. Core logic stays board-independent.
- **Parity tracking:** the table in Phase 5 is the source of truth for "how done are we".
- **Security:** offensive-security tooling — document responsible-use scope; keep AGPL intact.
- **Cost:** local Qwen for mechanical porting, a cloud model (Opus) for architecture and
  hard drivers.

## Working rules

- One module/coherent change per PR. **Never push to `main`** (enforced).
- Every PR: green CI + tests for new logic.
- `bruce-core` never imports `esp-hal`; hardware goes through a HAL trait.
- Track parity against a pinned upstream commit, not a moving target.

## Honest scale check

16 modules, ~410 files, a full WiFi/BLE/RF/NFC stack and an embedded JS interpreter. Full
parity is a long road and some of it may never be worth porting. The value is in the path:
every PR is shippable and CI-verifiable, so the project survives being picked up and put down.
