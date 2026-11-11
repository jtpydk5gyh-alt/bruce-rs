# Roadmap

Rewriting ~150k lines of C/C++ firmware is a long haul. The plan is to grow outward from
a tested, hardware-agnostic core, port one module at a time, and never merge code that
doesn't compile and pass CI. No phase depends on owning a board except where noted.

## Phase 0 — Foundations ✅ (this scaffold)

- [x] Cargo workspace: `bruce-hal`, `bruce-core`, `boards/*`.
- [x] Trait-based HAL and a host-tested core (menu navigation).
- [x] CI: `fmt` + `clippy -D warnings` + `cargo test`.
- [ ] `CONTRIBUTING.md`, issue templates, PR template.

## Phase 1 — First real target (needs a board to flash, not to build)

- [ ] Pick the first chip. **Recommended: an ESP32-C3/C6 (RISC-V)** — simplest toolchain.
- [ ] Add the `esp-hal` target, a `board-*` binary that boots and prints to the screen.
- [ ] CI job that cross-compiles the firmware for that target (compile-only, no hardware).
- [ ] Real `Display` + `Keyboard` drivers for that board.
- [ ] "Hello" menu running on device. **First milestone that requires the hardware.**

## Phase 2 — Core infrastructure

- [ ] Storage (SD / SPIFFS) behind `Storage`.
- [ ] Settings, persistence, boot flow.
- [ ] File browser and the module/menu framework matching Bruce's UX.

## Phase 3 — Feature modules (ported one by one, each with tests)

Ordered easiest-to-hardest, pure-logic first so most of it is testable without a board:

1. [ ] IR (parsers + send/receive) — lots of pure data, great starting module.
2. [ ] RF / Sub-GHz (CC1101).
3. [ ] NFC / RFID.
4. [ ] WiFi tooling.
5. [ ] BLE tooling.

## Phase 4 — Breadth

- [ ] Second and third boards (prove the HAL abstraction holds).
- [ ] Feature parity tracking table vs upstream Bruce.
- [ ] Web/USB flashing and release artifacts from CI.

## Working rules

- One module per pull request; never push to `main`.
- Every PR: green CI, and new logic ships with unit tests.
- Keep `bruce-core` free of `esp-hal`. Hardware goes through a HAL trait.
- Track parity against a specific upstream Bruce commit, not a moving target.

## Honest expectations

This is months of work, and the hard parts (radio timing, driver quirks) only reveal
themselves on real hardware. The goal of this structure is that progress is always
shippable and verifiable in CI, so the project survives being picked up and put down.
