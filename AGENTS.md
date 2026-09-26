# Madrid Lim site guidance

## Binding specification

`STYLE.md` is binding for product intent, layout, copy, tokens, accessibility, media, responsive behavior, and security. If another document conflicts with it, follow `STYLE.md`.

## Stack and structure

- Rust and Leptos CSR compile to WebAssembly with Trunk.
- `src/main.rs` mounts the app.
- `src/content.rs` is the source for visible project, run post, GPX and contact data.
- `src/components/mod.rs` implements the mounted editorial page: top bar, hero, marquee, work list, run reel, about, contact footer and easter eggs.
- Only `src/components/mod.rs` and `src/content.rs` are referenced by the mounted entry point. Old component, GL, and GPX parser files stay uncompiled and unlinked.
- `style/tokens.css` contains the Anton font face and the exact light and dark tokens from `STYLE.md`.
- `style/main.css` contains layout, motion and responsive styles.
- `assets/profile.jpg` is the post avatar. `assets/media/` holds cutouts, post images and full-length flyovers with posters. `assets/work/` holds real project screenshots. `assets/gpx/` holds downloadable routes. `assets/docs/` holds the redacted resume and grades; never commit unredacted originals. `assets/fonts/` holds self-hosted Anton (OFL).
- `assets/evidence/` is the old media set and is no longer referenced or copied.
- Old modules may remain on disk but must not be referenced, compiled, copied, or visible.

Do not add JavaScript frameworks, TypeScript, Tailwind, WebGL, canvas, external fonts, or external runtime requests. Production Rust must remain fallible around browser APIs. Do not use unsafe HTML or inline style attributes; set runtime styles through CSSOM.

## Content and persistence

Use concise factual copy. Do not use em or en dashes, visible AI wording, a phone number, date of birth, or student number. New-tab links require `rel="noopener noreferrer"`; mail links remain in the same tab. The only persistent user setting is the light or dark theme in local storage.

## Validation

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release
```

Also check the built CSP (the two loader hashes in `index.html` must match `dist/index.html` and a `--public-url /madrid-lim/` build), public URL behavior, hero word cycling and reduced motion, work row expand, show all and hover preview, easter eggs, contact mailto, redacted document downloads, flyover autoplay only in view and pause when hidden, sound and info toggles, GPX downloads, desktop and mobile overflow, keyboard interactions, both themes, 200 percent zoom, console errors, forbidden visible strings, and external network requests. GitHub Pages uses `/madrid-lim/`; local preview uses `/`. `trunk serve` injects a reload script that the CSP blocks, so preview release builds with a static server from `dist/`.

## Repository safety

Do not commit, push, create remotes, deploy, or modify sibling repositories. Do not delete existing files without explicit approval. Runtime output and screenshots belong under `/tmp`, not the repository.

Follow the authorship policy in `CONTRIBUTING.md`: never add AI or bot `Co-authored-by` trailers or generated-by lines to commits.
