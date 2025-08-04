# ClipDock

ClipDock is a tiny cross-platform clipboard manager built with a Rust + [Tauri](https://tauri.app/) backend and a Svelte + Tailwind frontend.

## Features

- **Automatic history** – a background thread watches the system clipboard and persists new text snippets to a SQLite database, keeping the list up to date without user interaction.
- **Lightweight storage** – only the newest 20 entries are retained; older clips are discarded to keep the database small. You can pin favorite clips so they remain at the top of the list.
- **Fuzzy search palette** – press a single hotkey to open an always-on-top palette where you can quickly search, select, or pin clips.
- **Global shortcut** – `Ctrl+Shift+V` toggles the palette from anywhere, making your clipboard history instantly accessible.

## Project Structure

```
├── src-tauri   # Rust backend (Tauri)
└── ui          # Svelte + Tailwind user interface
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (1.77 or newer)
- [Node.js](https://nodejs.org/) and [pnpm](https://pnpm.io/)
- The [Tauri CLI](https://tauri.app/v2/guides/getting-started/prerequisites) for your platform

### Development

```bash
pnpm install            # install frontend dependencies
cd src-tauri
cargo tauri dev         # run the app with live reload
```

### Building

```bash
cd src-tauri
cargo tauri build       # create a release binary/installer
```

The build step compiles the Svelte app and bundles it with the Rust backend into a native executable.

## License

Distributed under the MIT License. See `LICENSE` for more information.
