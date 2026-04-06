# Contributing to DevTracker

Thanks for your interest in contributing! DevTracker is a macOS menubar app built with Tauri v2 + React, and we welcome contributions of all kinds.

## Getting started

### Prerequisites

- macOS 13+ (required for NSWorkspace APIs)
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) 22+
- [pnpm](https://pnpm.io/) 9+

### Setup

```bash
git clone https://github.com/guilhermeseckert/devtracker.git
cd devtracker
pnpm install
pnpm tauri dev
```

### Project structure

```
src/                    # React frontend (TypeScript)
  components/           # UI components (shadcn/ui Luma)
  hooks/                # React Query hooks
  lib/                  # API wrappers, types, utilities
src-tauri/              # Rust backend
  src/tracking/         # Activity detection (app, branch, zoom, jira)
  src/db/               # SQLite schema, queries
  src/commands/          # Tauri IPC commands
```

## Development workflow

1. **Fork and clone** the repo
2. **Create a branch** from `main` for your change
3. **Make your changes** — the app hot-reloads with `pnpm tauri dev`
4. **Run checks** before committing:
   ```bash
   pnpm typecheck     # TypeScript
   pnpm check         # Biome lint
   cargo check --manifest-path src-tauri/Cargo.toml  # Rust
   ```
5. **Format your code**: `pnpm fix`
6. **Open a PR** against `main`

## Code style

- **Frontend**: Enforced by [Ultracite](https://github.com/haydenbleasel/ultracite) (Biome). Run `pnpm fix` to auto-format.
- **Rust**: Standard `rustfmt` conventions. The CI will check compilation.
- **File naming**: kebab-case for all frontend files (e.g., `time-block.tsx`, not `TimeBlock.tsx`)
- **Imports**: Sorted by Biome, use `@/` alias for src imports

## What to work on

Check the [Issues](https://github.com/guilhermeseckert/devtracker/issues) tab. Good first issues are labeled accordingly. Some areas that need help:

- **Linux/Windows support** — replace macOS-specific APIs with cross-platform alternatives
- **Idle detection** — detect screen lock via `CGSessionCopyCurrentDictionary`
- **Jira API integration** — fetch ticket titles, optionally auto-log hours
- **VS Code extension** — send active workspace path directly to DevTracker
- **Better Zoom detection** — more reliable meeting detection across Zoom versions
- **Tests** — unit tests for Rust tracking logic, React component tests

## Reporting bugs

Please open an issue with:
- macOS version
- DevTracker version
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
