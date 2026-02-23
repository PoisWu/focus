# Plan: Nature Focus Desktop App — TDD Full Implementation

Fullscreen Tauri v2 + Svelte + TypeScript app showing cycling nature photos with a ghost UI. Spotify playback is controlled via the `mpris` Rust crate (pure D-Bus, no `playerctl`). Clean Architecture enforced throughout; Unsplash key lives on the Rust side only. Development follows strict TDD: **Red → Green → Refactor** at every layer, working inside-out.

---

**TDD Order Principle**
```
Domain → Application → Infrastructure → Presentation
  ↑           ↑               ↑               ↑
Pure logic  Use cases   Adapters/IO    UI + Tauri commands
```
Each layer's tests are written **before** the implementation. Inner layers are tested with zero external dependencies; outer layers mock inward interfaces.

---

## Steps

### 1. Scaffold & Prerequisites
- Verify Linux deps: `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libdbus-1-dev`, `rustup`, Node LTS
- Run `npm create tauri-app@latest . -- --template svelte-ts`
- Commit scaffold baseline

### 2. Configure test infrastructure (do this before writing any feature code)

*Rust dev-dependencies* — add to `src-tauri/Cargo.toml`:
```toml
[dev-dependencies]
mockall    = "0.14.0"   # auto-generate mock structs from traits
tokio-test = "0.4"      # async test utilities
rstest     = "0.23"     # parametric tests + fixtures
```

*Frontend test stack* — install:
```bash
npm install -D vitest @testing-library/svelte @testing-library/user-event \
  @testing-library/jest-dom @sveltejs/vite-plugin-svelte jsdom \
  @vitest/ui @vitest/coverage-v8
```

*Vitest config* — update `vite.config.ts`:
- Add `svelte()` and `svelteTesting()` plugins
- Set `test.environment: 'jsdom'`, `test.setupFiles: ['./vitest-setup.ts']`

*`vitest-setup.ts`* — create at repo root:
- Import `@testing-library/jest-dom/vitest`
- Polyfill `window.crypto` with Node's `randomFillSync` (jsdom lacks WebCrypto, Tauri IPC needs it)
- Call `clearMocks()` in `afterEach` to prevent Tauri mock state leaking between tests

Add `"test": "vitest"` and `"test:ui": "vitest --ui"` to `package.json`

---

## LAYER 1 — Domain (pure logic, no I/O)

### 3. RED: Write domain tests first
- `src/domain/photo.test.ts`: assert `Photo` type has required fields; test any domain invariants (e.g. URL is non-empty)
- `src/domain/track.test.ts`: assert `Track` status is one of the three valid values
- `src-tauri/src/models.rs`: write `#[cfg(test)]` tests asserting `PhotoDto` and `TrackDto` serialize to correct JSON shapes

### 4. GREEN: Implement domain types
- `src/domain/photo.ts`: `type Photo = { id: string; url: string; photographer: string; profileUrl: string }`
- `src/domain/track.ts`: `type Track = { title: string; artist: string; status: 'Playing' | 'Paused' | 'Stopped' }`
- `src-tauri/src/models.rs`: `PhotoDto`, `TrackDto` with `#[derive(Serialize)]`
- `src-tauri/src/error.rs`: `AppError` enum with `thiserror`; `Unsplash`, `Mpris`, `NoPlayer` variants; `impl Serialize`

---

## LAYER 2 — Application (use cases; mock all ports)

### 5. RED: Write application port contracts and use case tests first
- `src/application/ports.ts`: define `PhotoRepository`, `SlideshowClock`, `MusicController` interfaces — these are the contracts mocks will implement
- `src/application/usecases/prefetchPhotos.test.ts`: inject a mock `PhotoRepository` returning fixed `Photo[]`; assert queue grows by batch size; assert it handles empty response gracefully
- `src/application/usecases/advanceSlide.test.ts`: assert queue shrinks by 1; assert `prefetchPhotos` is triggered when `queue.length < 3`; assert returns correct `{ current, queue }`

*Rust use-case tests* — in `#[cfg(test)]` inline modules:
- In `src-tauri/src/commands/photos.rs`: write a test asserting that `fetch_photos` delegates to an injectable HTTP client (mock with `mockall`) and maps the response to `Vec<PhotoDto>`
- In `src-tauri/src/commands/spotify.rs`: write tests asserting `spotify_play_pause` / `spotify_next` / `spotify_get_track` return correct `TrackDto` from a mock MPRIS interface, and return `AppError::NoPlayer` gracefully when no player is found

### 6. GREEN: Implement use cases
- `src/application/usecases/prefetchPhotos.ts`: accepts `PhotoRepository`, fetches batch, returns updated queue — pure, no side effects
- `src/application/usecases/advanceSlide.ts`: pops front of queue, triggers `prefetchPhotos` if low, returns `{ current, queue }` — pure, injectable
- `src-tauri/src/commands/photos.rs`: `fetch_photos` Tauri command reading `UNSPLASH_ACCESS_KEY`, calling Unsplash via `reqwest`, mapping to `Vec<PhotoDto>`
- `src-tauri/src/commands/spotify.rs`: three commands using `mpris::PlayerFinder` — `play_pause`, `next`, `get_metadata` → `TrackDto`

---

## LAYER 3 — Infrastructure (adapters; test with real or in-memory backends)

### 7. RED: Write infrastructure tests first
- `src/infrastructure/tauriPhotoRepository.test.ts`: use `mockIPC` from `@tauri-apps/api/mocks` to intercept `invoke('fetch_photos')`; assert returned array maps to `Photo[]` correctly
- `src/infrastructure/tauriMusicController.test.ts`: use `mockIPC` to stub `spotify_get_track`; assert `null` returned when backend throws `NoPlayer`
- `src/infrastructure/slideshowStore.test.ts`: use `get()` from `svelte/store`; assert store mutations (`setPhoto`, `togglePause`, `tickSecond`) produce correct state transitions
- `src/infrastructure/musicStore.test.ts`: assert `hudVisible` toggles correctly; assert `track` updates on `setTrack`
- `src/infrastructure/browserClock.test.ts`: use `vi.useFakeTimers()`; assert callback fires at correct intervals; assert polling stops when HUD is hidden

### 8. GREEN: Implement infrastructure
- `src/infrastructure/tauriPhotoRepository.ts`: implements `PhotoRepository` via `invoke<Photo[]>('fetch_photos')`
- `src/infrastructure/tauriMusicController.ts`: implements `MusicController`; catches `NoPlayer` → returns `null` from `getCurrentTrack`
- `src/infrastructure/slideshowStore.ts`: Svelte writable `{ current, queue, paused, secondsLeft }`
- `src/infrastructure/musicStore.ts`: Svelte writable `{ track: Track | null, hudVisible: boolean }`
- `src/infrastructure/browserClock.ts`: 1 s tick → `secondsLeft`; 30 s → `advanceSlide`; polls `getCurrentTrack` every 5 s only while `hudVisible`

---

## LAYER 4 — Presentation (components + Tauri commands; test with Testing Library + mockIPC)

### 9. RED: Write component tests first
- `src/presentation/components/PhotoFrame.test.ts`: assert `<img>` renders with correct `src`; assert `{#key photo.id}` change triggers re-render (new DOM node)
- `src/presentation/components/TimerDot.test.ts`: assert countdown value is displayed; assert correct `opacity` and `font-size` via computed styles
- `src/presentation/components/Attribution.test.ts`: assert photographer name renders; assert `<a>` has correct `href`
- `src/presentation/components/MusicHud.test.ts`: assert HUD not rendered when `hudVisible = false`; assert title + artist shown when visible; assert click on play button calls `playPause`
- `src/presentation/App.test.ts`: use `mockIPC` + `userEvent.keyboard`; assert `Escape` triggers close; `ArrowRight` advances slide; `Space` toggles pause; `m` toggles HUD

### 10. GREEN: Implement presentation
- `src/presentation/components/PhotoFrame.svelte`: `{#key photo.id}` + `transition:fade={{ duration: 800 }}`, `object-fit: cover`
- `src/presentation/components/TimerDot.svelte`: fixed bottom-right, `opacity: 0.2`, `font-size: 0.55rem`
- `src/presentation/components/Attribution.svelte`: fixed bottom-left, ghost style, `<a href={profileUrl}>`
- `src/presentation/components/MusicHud.svelte`: fixed top-center, `{#if hudVisible}` + `transition:fade={{ duration: 300 }}`; auto-hides after 3 s idle; hover hit-zone shows/hides
- `src/App.svelte`: keyboard handler; mount lifecycle; wires all components

### 11. DI root — no tests needed (pure wiring)
- `src/main.ts`: instantiate `TauriPhotoRepository`, `TauriMusicController`, `BrowserClock`; only file that imports across layer boundaries

### 12. Tauri config & CSS
- `src-tauri/tauri.conf.json`: `decorations: false`, `fullscreen: true`, CSP with `https://images.unsplash.com`
- `src-tauri/src/lib.rs`: register all four Tauri commands in `generate_handler![]`
- `src/app.css`: `overflow: hidden`, `background: #000`, no `backdrop-filter` (unreliable in WebKitGTK)

### 13. .env & .gitignore
- `.env`: `UNSPLASH_ACCESS_KEY=your_key_here`; add to `.gitignore`

---

## Verification
- `npm test` → all use-case, store, infrastructure, and component tests pass (zero Tauri runtime needed)
- `npm run test:ui` → Vitest browser UI for visual coverage inspection
- `npm run tauri dev` → fullscreen app, photos crossfade every 30 s, keyboard shortcuts work
- Press `m` or hover top area → MusicHud fades in; auto-hides after 3 s; Spotify not running → no crash
- `npm run tauri build` → `.deb` / AppImage in `src-tauri/target/release/bundle/`
- Test on both X11 and Wayland sessions (fullscreen + decoration behavior differs)

---

## Decisions
- **TDD order**: inside-out (Domain → Application → Infrastructure → Presentation); inner layers have zero external deps making Red→Green cycles fast
- **Rust mocking**: `mockall = "0.14.0"` with `#[cfg_attr(test, automock)]` on traits; command handlers stay thin (one-line delegation) because `tauri::State<T>` cannot be constructed in tests
- **Frontend Tauri mocking**: `mockIPC()` from `@tauri-apps/api/mocks` (official Tauri v2 utility); `clearMocks()` in `afterEach` is mandatory to prevent state leakage
- **jsdom crypto polyfill**: required in `vitest-setup.ts` as jsdom lacks `window.crypto` which Tauri IPC needs
- **`mpris` crate** (pure D-Bus, confirmed by ArchWiki Jan 2026) over `playerctl` CLI subprocess — no external binary dep; MPRIS2 is a frozen spec so the crate's version is stable
- **`rustls-tls`** for `reqwest` — avoids native OpenSSL complexity on Linux
- **`{#key photo.id}`** crossfade — simpler than two-image CSS overlap; easier to assert in tests (new DOM node = new `src` attribute)
- **HUD polling**: `getCurrentTrack` polled every 5 s only while `hudVisible` — no wasted D-Bus calls when hidden
- **Unsplash key on Rust side only** — never enters JS bundle; read via `std::env::var("UNSPLASH_ACCESS_KEY")`
