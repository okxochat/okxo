# Architecture

This describes the design as it currently stands. When it changes, update this file and add a record to [decisions/](decisions/) explaining why.

The interactive explorer, comparing four ways a message could be handled with timings and a crash simulator, is at https://claude.ai/artifact/5BUSrAsfjzgzQV5ix6PNHa. It's private: only the owner can open it unless it's shared.

## Goals

- Core Slack features: workspaces, channels, DMs, threads, reactions, unread counts, search.
- Agents are first-class: they join channels, read and post through the same path as people, and stream their replies live.
- Messages live on object storage through SlateDB, which keeps them cheap and durable. Metadata lives in a relational DB.

## Components (current target: option A)

```
client ──WebSocket──▶ okxo server ──WriteBatch──▶ SlateDB ──▶ object store (S3 / MinIO / local)
                         │  ▲
                         │  └─ in-process broadcast to subscribers (people and agents)
                         └──── membership and permission checks ──▶ SQLite / Postgres
```

One process owns one SlateDB and acks a send only once the write is durable. `ChannelOwner` and `Bus` are interfaces, so moving to option B (sharding) later doesn't change any callers.

## SlateDB facts the design relies on

Checked against slatedb 0.18.0 in November 2026.

- **One writer per database.** Opening a writer bumps `writer_epoch` in the manifest, and the old writer is fenced out on its next write.
- **Writes are batched.** A write lands in memory first, and the WAL is flushed to object storage every `flush_interval`. Waiting for that flush is optional.
- **Readers see only durable data.** A `DbReader` in another process sees only flushed data.
- **The PUT limit is about 3,500 per second per prefix.**

Still to confirm in the rustdoc ([todo.md](todo.md)): the default `flush_interval`, and whether `write()` waits until the data is durable by default.

## Data placement

| Relational (SQLite, later Postgres) | SlateDB |
|---|---|
| workspaces, users, sessions | messages |
| channels, memberships, roles | thread replies |
| agents, agent scopes, `agent_runs` | reactions |
| read cursors (`last_read_seq`) | channel head (latest seq) |
| shard leases (option B only) | idempotency keys |

## Keys

```
ch/{cid}/m/{seq:020}            message body (JSON for now)
ch/{cid}/t/{root}/{seq:020}     thread reply
ch/{cid}/r/{seq}/{emoji}/{uid}  reaction
ch/{cid}/head                   latest seq (u64, big-endian)
ch/{cid}/idem/{client_msg_id}   seq (u64, big-endian), TTL about 24h
```

Rules:
- **`{cid}` is a hyphenated UUID, always 36 characters**, so one channel's prefix can never overlap another's.
- **`seq` is zero-padded to 20 digits**, because SlateDB sorts keys by comparing bytes. Without the padding, `10` sorts before `9`.
- **One `WriteBatch` per message.** The message, `head` and `idem` keys commit together.
- **No dual writes.** The channel head exists only in SlateDB. The unread count is `head − last_read_seq`, worked out when it's read.
- **The single writer assigns seq** from a counter in memory, and on restart recovers it from `head`.

## Message path (option A)

1. The client sends `{channel_id, body, client_msg_id}`.
2. The server checks membership against a cache of the relational DB.
3. If the `idem` key exists, the server returns the existing message. This makes retries safe.
4. Otherwise it sets seq to head + 1 and commits one `WriteBatch` with the message, `head` and `idem` keys.
5. Once the write is durable, it acks the sender and broadcasts to subscribers.
6. The sender's client shows a local echo straight away and marks it sent when the ack arrives.

## Options considered

| Option | Summary | Status |
|---|---|---|
| A | Single node; ack once durable | **Chosen to start** |
| B | Channels sharded across owner nodes, one SlateDB per shard; Postgres leases and a pub/sub bus | Later, when one writer isn't enough |
| C | Show recipients first, confirm once stored | Rejected: recipients can see messages that later disappear |
| D | Local disk log in front of SlateDB | Rejected for now: an ack no longer means durable, and it's a second log to run |

## Agents

- An agent is a channel member with scoped permissions, checked the same way as a person's.
- Agents receive only durable messages: WebSocket for hosted agents, signed webhook for external ones, or an MCP server.
- Streaming replies: draft tokens go over the bus and are never saved, and the final reply is one write. Each run is a row in `agent_runs`, so a crashed run shows as "interrupted".
- When agents mention other agents, a hop counter limits the chain, and each run has a token budget.

## Open questions

- **Message IDs:** seq per channel only, or also a global ULID?
- **Read cursors:** debounce writes to Postgres, or move them to SlateDB?
- **Search:** a separate index (tantivy) fed by the durable stream or by SlateDB's change data capture?
- **Retention:** per-workspace TTL, range deletes.
- **Edits and deletes:** overwrite with a tombstone, or append edit events?
- **Multi-region:** one home region per shard, with readers elsewhere?
