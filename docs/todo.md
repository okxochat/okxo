# To do

Small one-off tasks. Anything bigger belongs in [features.md](features.md) or [walkthrough.md](walkthrough.md). Tick an item off when it's done, and clear out done items now and then.

## Open

- [ ] Decide how `scaffold` reaches `main`: a pull request, or merge directly.
- [ ] Check in the slatedb 0.18 rustdoc: the default `flush_interval`, and whether `write()` waits until data is durable (`WriteOptions` / `await_durable`).
- [ ] Add a `release` mise task (`cargo build --release`).
- [ ] Set up CI on GitHub Actions to run `mise run ci` on every push.

## Done

- [x] Install mise and Rust 1.99.1 (2026-11-06)
- [x] Activate mise in the shell
- [x] Rename `ChanneliD` to `ChannelId`
- [x] Restore `greeting()` in `main.rs` so `mise run ci` passes
- [x] `keys::idem` takes `&str` instead of `String`
- [x] `Key` newtype with private field, `AsRef<[u8]>` and a readable `Debug`
- [x] Move `okxo-store` from `creates/` to `crates/` (2026-11-09)
- [x] Push the scaffold to the `scaffold` branch (2026-11-10)
