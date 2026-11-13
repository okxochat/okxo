# Development

## Tooling

- **mise** manages the toolchain and repeated tasks. Its config is in [mise.toml](../mise.toml).
- **Rust 1.99.1** is pinned in `mise.toml`, with rustfmt and clippy.
- If `cargo` isn't found, mise isn't active in your shell. Either activate it (see [todo.md](todo.md)) or put `mise exec --` in front of commands.

## Commands

| Command | What it does |
|---|---|
| `mise run build` | Build all crates |
| `mise run run` | Run the `okxo` binary |
| `mise run test` | Run all tests |
| `mise run fmt` | Format all code |
| `mise run lint` | Check formatting, then run clippy with warnings treated as errors |
| `mise run ci` | Lint and test. **Run this before every commit.** |
| `cargo test -p okxo-store` | Test only one crate |
| `cargo test some_name` | Run only tests whose name contains `some_name` |
| `cargo test -- --nocapture` | Show `println!` output in tests |

## Layout

```
Cargo.toml          workspace: shared version, edition 2024, lints
mise.toml           toolchain and tasks
crates/
  okxo/             the binary (server, eventually)
  okxo-store/       message storage on SlateDB
docs/               this folder
```

A new crate goes in `crates/<name>` and uses workspace inheritance: `version.workspace = true` and so on, plus `[lints] workspace = true`. Copy [crates/okxo/Cargo.toml](../crates/okxo/Cargo.toml) as a template.

## Code style

- **Format with `cargo fmt`.** Don't hand-format.
- **Clippy warnings fail the build** (`-D warnings`). Fix them; only use `#[allow]` when you can write down the reason.
- **`unsafe` is forbidden** across the workspace.
- **Use newtypes for IDs**, for example `ChannelID(Uuid)`, so different kinds of ID can't be mixed up.
- **Errors:** each library crate gets one `thiserror` enum. Don't use `unwrap()` outside tests.
- **Async:** `tokio`. A lock held across `.await` must be `tokio::sync::Mutex`, not `std::sync::Mutex`.
- **Doc comments (`///`)** go on every public item.
- **Only add a dependency when the current step needs it.**

## Testing

- Unit tests sit in `#[cfg(test)] mod tests` at the bottom of the file they test.
- Storage tests use an in-memory object store (`object_store::memory::InMemory`), so they need no network or credentials.
- **See a new test fail before trusting it.** Break the code on purpose, check the test catches it, then fix it again.

## Git

- `main` is the default branch. Do work on a branch, for example `scaffold` or `store-keys`.
- Commit messages: a short summary line in the imperative ("Add key encoding"), then a blank line and the details if needed.
- **The repo is public.** Never commit `.env` files, keys or tokens. `.gitignore` covers `.env*` and `mise.local.toml`.
- Run `mise run ci` before committing.
