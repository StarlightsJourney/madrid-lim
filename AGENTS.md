# Madrid Lim site guidance

## Binding specification

`STYLE.md` is binding for product intent, layout, copy, tokens, accessibility, media, responsive behavior, and security. If another document conflicts with it, follow `STYLE.md`.

## Stack and structure

- Rust and Leptos CSR compile to WebAssembly with Trunk.
- `src/main.rs` mounts the app.
- `src/content.rs` is the source for visible project and contact data.
- `src/components/mod.rs` implements the mounted one-window portfolio.
- Only `src/components/mod.rs` and `src/content.rs` are referenced by the mounted entry point. Old component, GL, and GPX files stay uncompiled and unlinked.
- `style/tokens.css` contains the exact light and dark tokens from `STYLE.md`.
- `style/main.css` contains the document window, original pixel garden, drag, ticker, motion, and responsive component styles.
- `assets/profile.jpg` is the optimized real identity crop.
- `assets/evidence/` contains real images, poster frames, and route videos shown only in the evidence drawer. Video sources are created only after explicit play.
- Old modules may remain on disk but must not be referenced, compiled, copied, or visible.

Do not add JavaScript frameworks, TypeScript, Tailwind, WebGL, canvas, external fonts, or external runtime requests. Production Rust must remain fallible around browser APIs. Do not use unsafe HTML.

## Content and persistence

Use concise factual copy. Do not use em or en dashes, visible AI wording, a phone number, date of birth, or student number. New-tab links require `rel="noopener noreferrer"`; mail links remain in the same tab. The only persistent user setting is the light or dark theme in local storage.

## Validation

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release
```

Also check the built CSP, public URL behavior, drag bounds and reset, mobile drag disablement, ticker accessibility and reduced motion, critter feedback, poster-first video loading, desktop and mobile overflow, keyboard interactions, both themes, 200 percent zoom, console errors, forbidden visible strings, and external network requests. GitHub Pages uses `/madrid-lim/`; local Trunk preview uses `/`.

## Repository safety

Do not commit, push, create remotes, deploy, or modify sibling repositories. Do not delete existing files without explicit approval. Runtime output and screenshots belong under `/tmp`, not the repository.

Follow the authorship policy in `CONTRIBUTING.md`: never add AI or bot `Co-authored-by` trailers or generated-by lines to commits.
