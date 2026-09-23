# Madrid Lim

A compact career portfolio presented as one centered macOS-style document workspace. Projects are primary. Identity, academic context, contact links, selected experience, and real endurance evidence remain concise and factual.

`STYLE.md` is the binding product and visual specification.

## Stack

Rust, Leptos CSR, WebAssembly, Trunk, and plain CSS. The app uses system fonts and makes no external runtime requests.

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

For the Pages path, build into a separate directory so the root `dist/` stays intact for local preview: `trunk build --release --public-url /madrid-lim/ --dist /tmp/madrid-lim-pages-dist`, then inspect `/tmp/madrid-lim-pages-dist/index.html`.

## Interaction checks

1. Confirm the full window fits at 1440 by 900 and 1280 by 720 without page scroll.
2. Select each project and use project list arrow keys.
3. Switch among Projects, About, and Outside.
4. Toggle and reload both themes.
5. Open evidence, inspect the three real images, then test Escape, backdrop close, and focus restoration.
6. Press `/`, run every fixed command, test history arrows, Escape, and focus restoration.
7. Confirm external links use `noopener noreferrer`.
8. Check 390 by 844, 320 pixel width, and 200 percent zoom for one natural scroll and no horizontal overflow.
9. Inspect the console, CSP, network requests, release asset sizes, and absence of source maps.

## Content and media

Visible project and contact data live in `src/content.rs`. The mounted UI is implemented in `src/components/mod.rs`. Three optimized real images live in `assets/evidence/` and are inserted only when the evidence dialog opens. Old files can remain in the repository, but the entry point does not reference or compile old component, GL, or GPX modules.
