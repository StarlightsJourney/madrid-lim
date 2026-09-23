---
name: madrid-site-workflow
description: Verify the Madrid Lim personal website after changes to components, content, evidence assets, styles, CSP, or deployment.
triggers: [user]
allowed-tools: [read, grep, edit, exec, browser_preview]
---

# Madrid site workflow

Use this workflow after changes to the one-screen portfolio's components, content, evidence assets, styles, CSP, or deployment.

## Prerequisites

Check toolchain availability before testing:

```sh
rustc --version
cargo --version
rustup target list --installed
trunk --version
```

Prefix Rust commands with `PATH="$HOME/.cargo/bin:$PATH"` if cargo is missing. Install `wasm32-unknown-unknown` with rustup and install Trunk with `cargo install trunk --locked` if needed.

## Validation commands

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release --public-url /
trunk build --release --public-url /madrid-lim/
```

## Workflow

1. Open the site from `trunk serve`.
2. Use the mouse and keyboard to select projects in the main view.
3. Switch tabs inside the project drawer and verify the active tab updates.
4. Toggle the theme and verify persistence across reload.
5. Open the terminal, run the fixed commands, check history, press Escape to close, and verify focus returns to the trigger.
6. Open outside evidence, confirm lazy images load, press Escape to close, and verify focus returns to the trigger.
7. Click external links and confirm they open with `rel="noopener noreferrer"`.
8. Check responsive layout at 1440x900, 1280x720, 390x844, 320, and 200% zoom.
9. Check the console, network tab, CSP, and source maps.

## Handoff checklist

Mark each item as Verified, Working locally, Mocked, Environment-blocked, or Not implemented. Only use Verified after personally checking it.

- Desktop and mobile layout
- Mouse and keyboard project selection
- Tab switching in the project drawer
- Theme persistence across reload
- Terminal commands, history, Escape to close, and focus restore
- Outside evidence lazy images, Escape to close, and focus restore
- External links open safely
- Responsive layout at 1440x900, 1280x720, 390x844, 320, and 200% zoom
- Console, network, CSP, and source maps

If a check fails, report a screenshot, console logs, exact paths, expected versus observed behavior, and blocker class.

## Known limitations

| Area | Current limitation |
| --- | --- |
| frame-ancestors | GitHub Pages cannot emit the frame-ancestors header. |
| Evidence assets | Portrait evidence assets may need owner replacement. |
| Safari and iOS | Untested unless a workflow run explicitly checks them. |
| Legacy files | Source still includes unused legacy files locally, but they are not compiled or shipped. |
