# 0001: Start with a single node that acks once durable (option A)

- **Status:** accepted
- **Date:** 2026-11-05

## Context

SlateDB allows one writer per database and batches writes to object storage. We compared four ways to handle a message (see [architecture.md](../architecture.md#options-considered)): single node, sharded owners, optimistic delivery, and a local log in front of SlateDB.

## Decision

Start with option A. One process owns one SlateDB, and the server acks a message only once it is durable. The owner and fan-out sit behind interfaces (`ChannelOwner`, `Bus`), and every key starts with the channel id.

## Consequences

- **Good:** one process to run, and an ack always means the message is in object storage. Each channel has a total order at no extra cost.
- **Bad:** send latency can't drop below flush wait plus PUT time, and restarting the server stops all writes until it's back.
- **Later:** moving to option B (sharding) needs no change to keys or to how seq is assigned.

---

New records go in this folder as `NNNN-short-title.md`, using the same headings.
