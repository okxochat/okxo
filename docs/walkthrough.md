# Walkthrough: building okxo step by step

This is the order to build okxo in. Each step is small, has a clear goal and ends with a checkpoint you can run. Do the steps in order, because each one relies on the one before.

Status: `[x]` done · `[~]` in progress · `[ ]` not started

**Current step: 1.3, open SlateDB, `append` and `get`.**

---

## Phase 0: Tooling

- [x] **0.1 Install mise and Rust.** mise 2026.11.2 is at `~/.local/bin/mise` and pins Rust 1.99.1 in `mise.toml`.
- [x] **0.2 Set up the Cargo workspace and hello world.** The `crates/okxo` binary prints `Hello from okxo!`.
- [x] **0.3 Add mise tasks.** `build`, `run`, `test`, `fmt`, `lint` and `ci`. See [development.md](development.md).
- [x] **0.4 Activate mise in your shell** so `cargo` works directly. `cargo` now runs through the mise shims.

## Phase 1: Message store (`crates/okxo-store`)

The first real piece: saving and reading a channel's messages in SlateDB. The design is in [architecture.md](architecture.md#keys).

- [x] **1.1 Crate and types.** Define `ChannelID`, `Message` and `NewMessage`.
  *Checkpoint:* `channel_id_display_is_fixed_width` passes.
- [x] **1.2 Key encoding (`keys.rs`).** Write `msg(ch, seq)`, `head(ch)` and `idem(ch, client_msg_id)`, each building a key as bytes.
  *Checkpoint:* a test proves `msg(c, 9) < msg(c, 10)`. Remove the zero-padding and watch it fail.
- [ ] **1.3 Open SlateDB, `append` and `get`.** Add the `slatedb` and `tokio` dependencies. `append` assigns the next seq and writes the message, `head` and `idem` keys in one `WriteBatch`.
  *Checkpoint:* three appends give seqs 1, 2 and 3, using an in-memory object store.
- [ ] **1.4 Idempotency and restart tests.**
  *Checkpoint:* sending the same `client_msg_id` twice returns the same seq. After close and reopen, the next seq continues from where it left off.
- [ ] **1.5 Reading history.** `list_after(ch, after, limit)` and `list_latest(ch, limit)`.
  *Checkpoint:* with 12 messages, `list_after` returns them in order.
- [ ] **1.6 Concurrency and fencing.** 50 concurrent appends produce seqs exactly 1 to 50. Opening a second writer fences out the first.
  *Checkpoint:* both tests pass. Record what fencing did in [architecture.md](architecture.md).

## Phase 2: Metadata (relational DB)

- [ ] **2.1 Pick a library.** Probably `sqlx`, which works with both SQLite and Postgres. Record the choice in [decisions/](decisions/).
- [ ] **2.2 Write migrations** for `workspaces`, `users`, `channels`, `memberships`, `agents` and `read_cursors`.
- [ ] **2.3 Add a `MetadataStore`** with create and get for workspaces and channels, plus a membership check. Tests run against SQLite.

## Phase 3: Server (option A from the architecture)

- [ ] **3.1 HTTP server** with `axum` and a `/health` endpoint.
- [ ] **3.2 WebSocket connection** that authenticates with a dev token, for now.
- [ ] **3.3 Send a message.** Check membership, call `MessageStore::append`, then send the ack.
- [ ] **3.4 In-process broadcast** to every subscriber of the channel, using `tokio::sync::broadcast`.
- [ ] **3.5 Fetch history** over HTTP with `list_latest` and `list_after`.
- [ ] **3.6 Unread counts** from `head − last_read_seq`.

## Phase 4: Client

- [ ] **4.1 Pick a client.** A web client (React, Vite, TypeScript) or a terminal client. Record the choice in [decisions/](decisions/).
- [ ] **4.2 Channel list, message list and composer**, with a local echo for the sender's message until the ack arrives.

## Phase 5: Agents

- [ ] **5.1 Agents as members.** Add agent rows and scopes.
- [ ] **5.2 Mention trigger.** A message that mentions an agent notifies the agent runtime.
- [ ] **5.3 Streaming replies.** Draft tokens go over the bus and only the final message is saved. Track each run in `agent_runs`.
- [ ] **5.4 MCP server** exposing `read_history` and `post_message`.

## Phase 6: Slack core features

Threads, reactions, edits and deletes, DMs, search. See [features.md](features.md).

## Later

Sharding (option B), multi-region. See [architecture.md](architecture.md).
