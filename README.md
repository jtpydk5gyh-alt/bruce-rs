# bruce-rs

A ground-up rewrite of the [Bruce](https://github.com/BruceDevices/firmware) ESP32
offensive-security firmware in Rust, built on [`esp-hal`](https://github.com/esp-rs/esp-hal).

This is an independent reimplementation, not a code fork. It reuses Bruce's feature
set and is released under the same license (AGPL-3.0-or-later). All credit for the
original firmware and its design goes to the Bruce authors and contributors.

> **Status: early scaffolding.** Nothing talks to real hardware yet. See
> [ROADMAP.md](ROADMAP.md) for the plan and [ARCHITECTURE.md](ARCHITECTURE.md) for how
> the crates fit together.

## Why Rust

- Memory safety on a platform where C++ bugs mean a bricked or exploitable device.
- One hardware-agnostic core (`bruce-core`) tested on the host, reused across every board.
- A clean trait-based HAL so adding a board is implementing an interface, not forking logic.

## Layout

| Crate | Role |
|-------|------|
| `bruce-hal` | Traits every board implements: display, keyboard, radio, storage. |
| `bruce-core` | Menus, modules and app logic. No hardware deps, fully unit-tested. |
| `boards/*` | One crate per supported board, implementing `bruce-hal` on `esp-hal`. |

## Build

```sh
cargo test --workspace   # host logic + unit tests, no board needed
```

Firmware builds for real targets land in Phase 1 (see the roadmap).

## License

AGPL-3.0-or-later, matching upstream Bruce. See [LICENSE](LICENSE).
