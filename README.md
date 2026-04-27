# Husk

> Lightweight service aggregator. No Electron, no bloat.

Husk runs your web services (messengers, mail, dashboards) as isolated webviews
in a single native shell. Built on Tauri 2 with native OS WebViews — target
under 400MB RAM for 3-5 services, against ~2.8GB for an equivalent Electron
aggregator.

## Status

MVP0 in progress. Not yet usable. See [Roadmap](#roadmap).

## Stack

- Tauri 2.x (Rust + native WebView per platform)
- Svelte 5 + TypeScript + Vite
- Bun (package manager)

## Platforms

| OS      | Arch    | Engine     |
| ------- | ------- | ---------- |
| macOS   | aarch64 | WKWebView  |
| macOS   | x86_64  | WKWebView  |
| Windows | x86_64  | WebView2   |
| Linux   | x86_64  | WebKitGTK  |

Primary target during MVP0: **macOS aarch64**. Other platforms validated in CI
but not performance-tuned until MVP1.

## Development

Prerequisites: Bun ≥ 1.2, Rust stable, plus
[Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
bun install
bun run tauri:dev
```

Build a production bundle:

```bash
bun run tauri:build
```

## Roadmap

MVP0 (current):

1. Bootstrap, license, SPDX headers
2. Service domain model + storage adapter
3. Static sidebar from store
4. Webview switching
5. Isolated sessions (gate: 2x WhatsApp simultâneos)
6. Service management screen (CRUD + reorder)
7. Global shortcuts mapped by position
8. System tray + single instance
9. Notifications bridge
10. Persistence + last active service
11. CI workflow
12. Release workflow
13. Code signing (conditional)
14. Polish + docs

MVP1+: Cmd+K omnibar, per-service custom CSS, multi-window, cloud sync.

## License

Apache 2.0 © Auditore. Maintained by [Édipo Maciel Bento Rosa](https://github.com/shuhikari).
