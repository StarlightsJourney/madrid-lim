---
name: madrid-site-workflow
description: Verify the Madrid Lim portfolio after changes to components, content, media, styles, CSP, or deployment.
triggers: [user]
allowed-tools: [read, grep, edit, exec, browser_preview]
---

# Madrid site workflow

Use this workflow after changes to the portfolio window, garden, drag behavior, ticker, profile, evidence assets, CSP, or deployment.

## Prerequisites

```sh
rustc --version
cargo --version
rustup target list --installed
trunk --version
ffmpeg -version
```

## Validation commands

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release --public-url /
trunk build --release --public-url /madrid-lim/ --dist /tmp/madrid-lim-pages-dist
```

## Workflow

1. Open the root release from a detached server and prove it responds with two curl requests.
2. Verify exactly six projects with pointer, Arrow keys, Home, and End.
3. Drag blank titlebar space to all viewport edges. Verify full bounds, control clicks do not drag, and double-click resets to center.
4. At 800 pixels and below, verify drag and transform offsets are disabled.
5. Verify the garden is visible only outside the neutral window in light and dark themes. Click both original critters and confirm `found me.` hides after about two seconds.
6. Verify one screen-reader technology list and two `aria-hidden` ticker tracks. Check hover and focus pause, 32 second left-to-right motion, and reduced-motion static single-track presentation.
7. Open evidence and confirm only images and posters request. Press one play button and confirm only its MP4 begins requesting. Check muted, controls, playsinline, no autoplay, captions, Escape, and focus restoration.
8. Open the terminal with `/`, run fixed commands, test history, Escape, and focus restoration.
9. Check 1440x900, 1280x720, 800x900, 390x844, 320x568, 200 percent zoom, dragged state, critter bubble, and evidence posters in both themes.
10. Check root and Pages CSP, console, network, source maps, asset sizes, external requests, forbidden strings, secrets, and identifiers.

Store screenshots under `/tmp/madrid-lim-shots/garden/`. Report initial release bytes excluding evidence, profile size, each video and poster size, and combined video size.

## Known limitations

| Area | Current limitation |
| --- | --- |
| frame-ancestors | GitHub Pages cannot emit the `frame-ancestors` response header. |
| Safari and iOS | Untested unless a workflow run explicitly checks them. |
| Legacy files | Unused legacy files remain locally but are not compiled or shipped. |
