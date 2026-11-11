# Contributing

Thanks for your interest in bruce-rs. A few rules keep the project sane.

## Golden rule: no direct commits to `main`

`main` is protected. Nobody — maintainers included — pushes to it directly. All changes
land through a pull request that passes CI.

```sh
git switch -c feat/ir-nec-decoder
# ... work ...
git push -u origin feat/ir-nec-decoder
# open a PR
```

## Before you open a PR

- `cargo fmt --all` — formatting is enforced.
- `cargo clippy --workspace --all-targets -- -D warnings` — no warnings.
- `cargo test --workspace` — green, and new logic comes with tests.

## Scope

- One module or one coherent change per PR. Small PRs get reviewed; giant ones rot.
- Keep `bruce-core` free of `esp-hal`. Hardware access goes through a `bruce-hal` trait,
  so the logic stays testable in CI without a board.

## Commit messages

- Imperative, descriptive. Explain the *why* when it isn't obvious.

## Licensing

By contributing you agree your work is licensed under AGPL-3.0-or-later, like the rest
of the project and like upstream Bruce.
