# Rules for Claude

Claude Code loads this file through the root `CLAUDE.md`. Edit it to change how Claude works in this repo.

## The user is learning Rust

- The goal is for the user to **write the code themselves**. By default, explain and give step-by-step guidance and code to type. Only edit `.rs` files when the user explicitly asks you to ("make those fixes", "write it for me").
- **Go one step at a time**, following [walkthrough.md](walkthrough.md): the goal, the code, then a checkpoint the user can run. Don't jump ahead.
- **Explain for someone new to Rust.** Compare with Python or JavaScript where it helps. Explain new syntax the first time it appears: ownership and borrowing, traits, macros, `?`, lifetimes.
- When the user's code has errors, **point to the line and explain the error** before offering to fix it.

## Never overwrite the user's edits

- If a file differs from what you last saw, assume the user changed it on purpose. Ask before you revert or rewrite it.
- Look at a file before deleting, moving or overwriting it.

## Tooling

- Run Rust commands through mise: `mise run <task>`, or `mise exec -- cargo ...`. Never install tools globally with Homebrew or rustup when mise can manage them.
- Run `mise run ci` before saying any code task is done. Report failures as they are.
- Add a dependency only when the current step needs it, using `cargo add` inside the crate's folder.

## Git and GitHub

- Commit and push only when the user asks.
- Never commit straight to `main`. Use a branch, and ask before merging.
- The repo is **public**. Check what's being committed for secrets (`.env`, keys, tokens) before every commit.

## Keep the docs current

- When a walkthrough step is finished, tick it in [walkthrough.md](walkthrough.md) and move the "Current step" marker.
- When a small task comes up, add it to [todo.md](todo.md). Tick it off when it's done.
- When a design decision is made, add `docs/decisions/NNNN-title.md` and update [architecture.md](architecture.md).
- When a feature's status changes, update [features.md](features.md).

## Design

- Follow [architecture.md](architecture.md). Raise it when something would go against it, such as a dual write, a key without the channel prefix, or a second writer.
- Check SlateDB API details against docs.rs for the pinned version. Don't rely on memory.
