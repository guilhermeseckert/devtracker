# DevTracker

A macOS menubar app that passively tracks your developer activity and stores it locally for Jira hour reporting.

## What it tracks

| Signal | Method | Permissions |
|--------|--------|-------------|
| Active app | `NSWorkspace` API | None |
| VS Code branch | Reads `.git/HEAD` from workspaces | Filesystem |
| Multiple branches | Scans all repos with recent activity (Claude Code support) | Filesystem |
| Jira ticket | Regex extraction from branch names | None |
| Zoom meetings | Process detection (`caphost`, `ZoomAudioDevice`) | None |

## Features

- **Menubar app** — lives in your macOS menu bar, click to open
- **Today timeline** — see all activity blocks for any day with day navigation
- **Summary** — hours grouped by Jira ticket or by repo, with week/month navigation
- **Export** — generate a text report, copy to clipboard for Jira
- **Manual tagging** — click any block to assign/change Jira ticket
- **Sleep detection** — automatically handles laptop sleep, won't count idle time
- **Multi-branch** — tracks all active branches when using VS Code or terminals
- **All local** — SQLite database, no cloud, no telemetry

## Install

Download the latest `.dmg` from [Releases](../../releases), open it, and drag DevTracker to Applications.

Or build from source:

```bash
git clone https://github.com/guilhermeseckert/devtracker.git
cd devtracker
pnpm install
pnpm tauri build
```

The built `.app` will be in `src-tauri/target/release/bundle/macos/`.

## Development

```bash
pnpm install
pnpm tauri dev
```

## Stack

- **Tauri v2** — Rust backend, native macOS tray
- **React 19** + **TypeScript** — frontend UI
- **shadcn/ui** (Luma) — component library
- **SQLite** (rusqlite) — local storage
- **Tailwind CSS v4** — styling
- **Biome** (Ultracite) — linting and formatting

## Data

All data is stored locally at:

```
~/Library/Application Support/com.devtracker.DevTracker/tracker.db
```

You can query it directly with `sqlite3`.

## License

MIT
