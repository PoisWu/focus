# Focus — Nature Focus Desktop App

A fullscreen Tauri v2 + Svelte + TypeScript app showing cycling nature photos with Spotify playback control via MPRIS2. Built following strict TDD (Test-Driven Development) practices with Clean Architecture.

## Features

- **Fullscreen nature photos** from Unsplash, cycling every 30 seconds
- **Spotify playback control** via MPRIS2/D-Bus (Linux)
- **Ghost UI** with photographer attribution and timer
- **Music HUD** (press `m`) for controlling playback
- **Keyboard shortcuts**:
  - `Escape` — Close app
  - `Space` — Pause/resume slideshow
  - `→` (Right Arrow) — Next photo
  - `m` — Toggle music HUD

## Prerequisites

- Linux with `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libdbus-1-dev`
- Rust (via `rustup`)
- Node.js LTS
- Unsplash API access key

## Setup

1. Clone the repository
2. Create `.env` file with your Unsplash API key:
   ```
   UNSPLASH_ACCESS_KEY=your_key_here
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Run in development:
   ```bash
   npm run tauri dev
   ```
5. Build for production:
   ```bash
   npm run tauri build
   ```

## Testing

- Frontend tests: `npm test`
- Rust tests: `cd src-tauri && cargo test`
- Test UI: `npm run test:ui`

## Architecture

The app follows Clean Architecture with strict TDD:

- **Domain** — Pure types (`Photo`, `Track`)
- **Application** — Use cases (`prefetchPhotos`, `advanceSlide`)
- **Infrastructure** — Adapters (Tauri IPC, Svelte stores)
- **Presentation** — UI components (Svelte)

All layers are tested independently with zero external dependencies in inner layers.

## Tech Stack

- **Frontend**: Vite, Svelte 5, TypeScript
- **Backend**: Tauri v2, Rust
- **Testing**: Vitest, @testing-library/svelte, mockall, rstest
- **APIs**: Unsplash (photos), MPRIS2 (Spotify control)

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## License

MIT
