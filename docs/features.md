# Features

Status: `planned` · `in progress` · `done` · `later`

For the build order, see [walkthrough.md](walkthrough.md).

## Core chat

| Feature | Status | Notes |
|---|---|---|
| Message store (append, get, list) | in progress | Phase 1. `crates/okxo-store` |
| Workspaces and users | planned | Phase 2. Relational DB |
| Channels (public, private) and membership | planned | Phase 2 |
| Send and receive in real time (WebSocket) | planned | Phase 3. Option A |
| Message history and paging | planned | `list_latest` and `list_after` |
| Unread counts | planned | `head − last_read_seq` |
| Threads | planned | `ch/{cid}/t/{root}/{seq}` |
| Reactions | planned | `ch/{cid}/r/{seq}/{emoji}/{uid}` |
| Edit and delete messages | planned | Approach still open. See architecture |
| Direct messages | planned | A DM is a private channel with 2 members |
| Mentions and notifications | planned | |
| Search | later | Separate index. See architecture |
| File uploads | later | Stored on object storage, with metadata in SQL |

## Agents

| Feature | Status | Notes |
|---|---|---|
| Agents as channel members, with scopes | planned | Phase 5 |
| Mention triggers | planned | |
| Streaming replies (unsaved drafts, final reply saved once) | planned | |
| `agent_runs` tracking, with interrupted and retry | planned | |
| MCP server (`read_history`, `post_message`, `react`) | planned | |
| Webhooks for external agents | later | |
| Loop guard and token budgets | planned | |

## Platform

| Feature | Status | Notes |
|---|---|---|
| Sharding (option B) | later | When one writer isn't enough |
| Multi-region | later | |
| Retention policies | later | TTL and range deletes |

## Feature notes

For a feature that needs more than a table row, add a section here with its goal, design sketch and open questions. Once it's built, move the design into [architecture.md](architecture.md).
