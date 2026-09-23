# Madrid Lim

A compact career portfolio presented as one draggable macOS-style document workspace in an original pixel garden. Projects remain primary, with concise identity, academic context, contact links, experience, and real endurance evidence.

`STYLE.md` is the binding product and visual specification.

## Stack

Rust, Leptos CSR, WebAssembly, Trunk, and plain CSS. The app uses system fonts, local media, CSS pixel art, and no external runtime requests.

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
trunk serve --address 127.0.0.1 --port 8080
```

Open `http://127.0.0.1:8080`.

## Validation

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release
```

For the Pages path, build into a separate directory: `trunk build --release --public-url /madrid-lim/ --dist /tmp/madrid-lim-pages-dist`, then inspect `/tmp/madrid-lim-pages-dist/index.html`.

## Interaction checks

1. Confirm the full window fits at 1440 by 900 and 1280 by 720.
2. Select all six projects with pointer and listbox keys.
3. Drag the window to each edge, check bounds, double-click blank titlebar space to reset, and confirm controls do not drag.
4. Confirm drag is disabled at 800 pixels and below.
5. Verify the ticker has one screen-reader list, hidden duplicate visual tracks, hover and focus pause, and a static reduced-motion layout.
6. Toggle and reload both themes. Check the garden and original critters in each.
7. Open evidence and confirm images and video posters load. Confirm MP4 bytes load only after each play button is pressed.
8. Press `/`, run every fixed command, test history, Escape, and focus restoration.
9. Check 390 by 844, 320 by 568, and 200 percent zoom for natural scroll and no horizontal overflow.
10. Inspect console, CSP, network requests, release asset sizes, and absence of source maps or external requests.

## Content and media

Visible project and contact data live in `src/content.rs`. The mounted UI is in `src/components/mod.rs`. `assets/profile.jpg` is a dedicated optimized crop. Three real images, three real route video excerpts, and their poster frames live under `assets/evidence/`. Evidence nodes enter the DOM only when the drawer opens, and video sources enter only after explicit play. Old files remain unreferenced and are not compiled or copied.
