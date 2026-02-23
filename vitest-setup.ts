import "@testing-library/jest-dom/vitest";
import { afterEach } from "vitest";
import { clearMocks } from "@tauri-apps/api/mocks";
import { webcrypto } from "node:crypto";

// Polyfill window.crypto for jsdom (Tauri IPC needs it)
if (!globalThis.crypto) {
  // @ts-ignore
  globalThis.crypto = webcrypto;
}

// Clear Tauri mocks after each test to prevent state leakage
afterEach(() => {
  clearMocks();
});
