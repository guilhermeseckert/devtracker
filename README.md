# DevTracker

A privacy-first macOS menubar app that passively tracks your developer activity. Know exactly where your time goes — no manual timers, no cloud sync, no telemetry.

## Why?

Ever been asked "what did you work on this month?" and drawn a blank? DevTracker runs silently in your menu bar, recording which apps you use, which git branches you're on, and when you're in meetings. At the end of the week or month, you get a clean breakdown — by project, by ticket, by repo.

All data stays on your machine. Always.

## What it tracks

| Signal | How | Permissions needed |
|--------|-----|--------------------|
| **Active app** | macOS `NSWorkspace` API | None |
| **Git branches** | Reads `.git/HEAD` from VS Code workspaces | Filesystem only |
| **Multiple branches** | Scans all repos with recent activity | Filesystem only |
| **Jira tickets** | Regex extraction from branch names (e.g. `feature/PROJ-123`) | None |
| **Zoom meetings** | Process detection (`caphost`, `ZoomAudioDevice`) | None |

No accessibility permissions. No screen recording. No keylogging. Just lightweight polling every 30 seconds.

## Features

- **Menubar app** — lives in your macOS menu bar, click to toggle
- **Today timeline** — color-coded activity blocks with day-by-day navigation
- **Summary view** — hours grouped by Jira ticket or by repo, with week/month navigation
- **Export** — one-click report generation, copy to clipboard
- **Manual tagging** — click any block to assign or change a Jira ticket
- **Sleep detection** — laptop sleep and idle gaps are automatically excluded
- **Multi-branch** — supports workflows with multiple repos open simultaneously (VS Code, Claude Code, terminal)
- **Navigate history** — browse any past day, week, or month

## Install

### Download

Grab the latest `.dmg` from the [Releases](https://github.com/guilhermeseckert/devtracker/releases) page. Open it, drag DevTracker to Applications, done.

> On first launch, macOS may show "unidentified developer" — right-click the app and select Open, then confirm.

### Build from source

Requires macOS 13+, [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/) 22+, and [pnpm](https://pnpm.io/).

```bash
git clone https://github.com/guilhermeseckert/devtracker.git
cd devtracker
pnpm install
pnpm tauri build
```

The `.app` and `.dmg` will be in `src-tauri/target/release/bundle/macos/`.

## Development

```bash
pnpm install
pnpm tauri dev
```

Hot-reloads both frontend (Vite) and backend (Cargo). See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

## Tech stack

| Layer | Technology |
|-------|-----------|
| Framework | [Tauri v2](https://tauri.app/) |
| Backend | Rust |
| Frontend | React 19, TypeScript |
| UI | [shadcn/ui](https://ui.shadcn.com/) (Luma preset) |
| Styling | Tailwind CSS v4 |
| Database | SQLite (via rusqlite) |
| Linting | [Biome](https://biomejs.dev/) via [Ultracite](https://github.com/haydenbleasel/ultracite) |

## How it works

DevTracker polls every 30 seconds to check:
1. Which app is in the foreground
2. Which git branch is active (if in VS Code or a terminal)
3. Whether a Zoom meeting is running

It only writes a new database row when the **state changes** — so switching from Chrome to VS Code creates a new block, but staying in VS Code for 2 hours is just one row. This keeps the database tiny (~50-100 rows per day).

Data is stored in a local SQLite file at:
```
~/Library/Application Support/com.devtracker.DevTracker/tracker.db
```

You can query it directly:
```bash
sqlite3 ~/Library/Application\ Support/com.devtracker.DevTracker/tracker.db \
  "SELECT jira_ticket, ROUND(SUM((julianday(ended_at) - julianday(started_at)) * 24), 1) as hours
   FROM activities WHERE started_at >= '2026-04-01'
   GROUP BY jira_ticket ORDER BY hours DESC;"
```

## Roadmap

- [ ] Idle detection via screen lock
- [ ] Jira API integration (fetch ticket titles, auto-log hours)
- [ ] VS Code extension for direct workspace reporting
- [ ] Editable time blocks (split, merge, drag boundaries)
- [ ] Linux and Windows support
- [ ] Slack/Teams call detection
- [ ] Weekly trends and analytics

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

[MIT](LICENSE)
