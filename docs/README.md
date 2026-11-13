# okxo docs

okxo is a Slack-style team chat with built-in AI agents. Messages are stored in [SlateDB](https://slatedb.io) on object storage. Workspaces, users, channels and permissions are kept in a relational database (SQLite, later Postgres).

| File | What it's for | When to update it |
|---|---|---|
| [walkthrough.md](walkthrough.md) | The step-by-step path for building okxo, and where we are on it | When you finish a step |
| [architecture.md](architecture.md) | How the system works now: components, data placement, message path | When the design changes |
| [decisions/](decisions/) | One short file per important decision, and why it was made | When a design choice is made |
| [features.md](features.md) | Feature backlog with status | When a feature is planned, started or shipped |
| [todo.md](todo.md) | Small one-off tasks | When something small comes up, or gets done |
| [development.md](development.md) | Tooling, commands, code style, testing, git workflow | When a convention changes |
| [claude-rules.md](claude-rules.md) | How Claude should work in this repo | When you want Claude to behave differently |

Claude Code reads [claude-rules.md](claude-rules.md) automatically through the root [CLAUDE.md](../CLAUDE.md).
