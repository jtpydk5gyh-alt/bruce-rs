# Roadmap

Rewriting ~150k lines of C/C++ firmware is a long haul. The strategy is to grow outward
from a tested, hardware-agnostic core, port one module at a time, and never merge code
that doesn't compile and pass CI. Except where a step explicitly says "on device", every
task is doable in GitHub Actions without owning a board.

**Legend:** ☐ todo · ☑ done · 🔌 needs physical hardware to verify.

---

## Phase 0 — Foundations ☑

The skeleton everything else hangs off.

- ☑ Cargo workspace: `bruce-hal`, `bruce-core`, `boards/*`.
- ☑ Trait-based HAL (`Display`, `Keyboard`, `Storage`, `Board`).
- ☑ Host-tested core (menu navigation with a unit test).
- ☑ CI: `cargo fmt --check` + `clippy -D warnings` + `cargo test`.
- ☐ `CONTRIBUTING.md`, PR template, `CODEOWNERS`.
- ☐ Branch protection on `main` (no direct pushes, PR-only).
- ☐ Issue templates (bug / feature / board-support).

**Exit criteria:** green CI on a PR, `main` protected, docs in place.

---

## Phase 1 — First real target

Goal: one board that boots Rust firmware and draws a menu. This is where hardware first
matters — everything up to the flashing step is CI-only.

- ☐ Pick the first chip. **Recommended: ESP32-C3 or C6 (RISC-V).** The RISC-V Rust
  target is upstream and stable; Xtensa (ESP32 / S2 / S3) needs the `esp-rs` toolchain
  fork and adds friction. Start on RISC-V, bring Xtensa in Phase 4.
- ☐ Add `esp-hal`, `esp-backtrace`, `esp-println`; set up `.cargo/config.toml` with the
  target and the `espflash` runner.
- ☐ Pin the toolchain and add the target to `rust-toolchain.toml`.
- ☐ CI job: cross-compile the firmware for that target (compile-only, no device).
- ☐ `board-*` binary that boots, inits clocks and prints over serial.
- ☐ Real `Display` driver (likely `mipidsi` over SPI) behind `bruce-hal::Display`.
- ☐ Real `Keyboard` driver behind `bruce-hal::Keyboard`.
- 🔌 ☐ `bruce-core` menu rendered and navigable on the physical device. **Milestone 1.**

**Exit criteria:** `cargo run` flashes the board and you can move the menu cursor on screen.

---

## Phase 2 — Core infrastructure

The plumbing every feature module will lean on. Mostly testable on the host.

- ☐ `Storage` trait impl: SD card (SPI) and/or internal flash (SPIFFS/FAT).
- ☐ Settings: load/save config, defaults, persistence across reboots.
- ☐ Boot flow: splash, main menu, module dispatch matching Bruce's UX.
- ☐ File browser (list, open, pick) — pure logic over the `Storage` trait, host-tested.
- ☐ Logging/serial diagnostics and a panic handler that survives in the field.
- ☐ Async runtime decision: evaluate `embassy` vs bare `esp-hal` loop; document the choice.

**Exit criteria:** boot → menu → file browser works on device; settings persist.

---

## Phase 3 — Feature modules

Ported one per PR, ordered easiest-to-hardest so the pure-logic parts (parsers, packet
formats) land first and are fully unit-tested before any radio is involved.

### 3.1 — IR ☐
Great starter: mostly data. Decode/encode NEC, RC5, Sony, Samsung, etc.
- ☐ Protocol parsers + encoders in `bruce-core` (host-tested against capture fixtures).
- ☐ `Ir` HAL trait (send/receive) + driver.
- 🔌 ☐ TV-B-Gone-style send and raw capture on device.

### 3.2 — Sub-GHz / RF ☐
- ☐ CC1101 driver behind an `Rf` trait.
- ☐ Signal record/replay, frequency scan; `.sub`-style file format in `bruce-core`.
- 🔌 ☐ Capture and replay a real remote.

### 3.3 — NFC / RFID ☐
- ☐ PN532 / RC522 driver behind an `Nfc` trait.
- ☐ Tag read/write/emulate logic; dump file formats (host-tested).
- 🔌 ☐ Read and clone a test tag.

### 3.4 — WiFi tooling ☐
- ☐ `esp-wifi` integration behind a `Wifi` trait.
- ☐ Scanner, beacon/probe tooling, captive-portal/evil-portal, packet-level features.
- 🔌 ☐ Scan and run a portal on device.

### 3.5 — BLE tooling ☐
- ☐ BLE stack integration; scanner and spam/advertising features.
- 🔌 ☐ Scan and advertise on device.

**Exit criteria per module:** logic unit-tested in CI, driver compiles for the target,
feature verified on hardware, parity entry updated.

---

## Phase 4 — Breadth & release

- ☐ Second and third boards, including the first Xtensa (ESP32-S3) board — proves the
  HAL abstraction holds and the `esp-rs` toolchain works in CI.
- ☐ Board-support matrix in the README, tracked against a pinned upstream Bruce commit.
- ☐ Feature-parity table vs upstream, updated per module.
- ☐ Release pipeline: CI builds signed `.bin` artifacts per board on tag.
- ☐ Web/USB flasher page (like Bruce's) or clear `espflash` instructions.
- ☐ Reproducible builds and a documented flashing guide.

**Exit criteria:** `git tag vX.Y.Z` produces downloadable firmware for every supported board.

---

## Cross-cutting tracks (run throughout)

- **Testing:** every bit of logic ships with unit tests; capture-based fixtures for
  protocols; `clippy -D warnings` stays green. Target: core logic independent of a board.
- **Docs:** `ARCHITECTURE.md` kept honest; each module gets a short design note.
- **Parity tracking:** a living table mapping each Bruce feature to its Rust status
  (missing / in progress / done / verified on hardware).
- **Security:** this is offensive-security tooling — document responsible-use scope and
  keep the AGPL obligations intact in every release.

---

## Working rules

- One module (or one coherent change) per pull request. **Never push to `main`.**
- Every PR: green CI, and new logic ships with unit tests.
- `bruce-core` never imports `esp-hal`; hardware always goes through a HAL trait.
- Track parity against a specific upstream commit, not a moving target.
- Reserve a cloud model (Opus) for architecture and hard drivers; use the local model
  for mechanical porting to keep cost down.

## Honest expectations

This is months of work, and the hard parts — radio timing, driver quirks, the Xtensa
toolchain — only surface on real hardware. This structure exists so progress is always
shippable and verifiable in CI, and the project survives being picked up and put down.
