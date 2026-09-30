# Madrid Lim

A bold editorial career portfolio: a cycling headline hero, selected work with expandable case notes, a reel of run posts with full route flyovers and GPX downloads, and a short about. Projects remain primary.

`STYLE.md` is the binding product and visual specification.

## Stack

Rust, Leptos CSR, WebAssembly, Trunk, and plain CSS. The app uses self-hosted Anton for display type, system fonts for body text, local media, and no external runtime requests.

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
git config core.hooksPath .githooks
trunk build --release
cd dist && python3 -m http.server 8080 --bind 127.0.0.1
```

Open `http://127.0.0.1:8080`. Preview release builds from `dist/` like this; `trunk serve` injects a reload script that the CSP blocks.

## Updating the site

The live site is https://starlightsjourney.github.io/madrid-lim/. Every merge into `main` rebuilds and publishes it through `.github/workflows/deploy.yml` in a few minutes. There is no manual deploy.

| To change | Edit |
| --- | --- |
| Project text, run posts, stats, GPX links, contact links, email, downloadable documents | `src/content.rs` |
| Layout and behaviour: hero lines, sections, easter eggs, contact form | `src/components/mod.rs` |
| Colours and font | `style/tokens.css` |
| Spacing, sizes, motion, mobile layout | `style/main.css` |
| Photos, cutouts, flyovers and posters | `assets/media/` |
| Project screenshots | `assets/work/` |
| GPX routes | `assets/gpx/` |
| Resume and grades | `assets/docs/` |

Workflow:

1. Create a branch: `git checkout -b my-change`.
2. Edit, then preview with `trunk build --release` and the static server above.
3. Run the validation commands below.
4. Commit, push and open a pull request: `git push -u origin my-change && gh pr create --fill`.
5. Merge it: `gh pr merge --merge --delete-branch`, or use the Merge button on GitHub.

Notes:

- Images are WebP. Videos are H.264 at about 432 by 960 with `+faststart`, plus a WebP poster frame taken from the video. Reference every new file in `src/content.rs`; `cargo test` fails if a referenced file is missing. A new folder under `assets/` also needs a `copy-dir` line in `index.html`.
- Never commit an original resume or transcript. Remove the phone number, student number and date of birth, and flatten the PDF to images first.
- `index.html` pins the Trunk loader in the CSP with two `sha256` hashes, one for `/` and one for `/madrid-lim/`. They stay valid for normal edits. After upgrading Trunk or renaming the crate, rebuild both targets and replace the hashes.
- Commits must not credit AI assistants or bots as co-authors. The hooks in `.githooks/` enforce this; see `CONTRIBUTING.md`.

## Validation

```sh
cargo fmt --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test
trunk build --release
```

For the Pages path, build into a separate directory: `trunk build --release --public-url /madrid-lim/ --dist /tmp/madrid-lim-pages-dist`, then inspect `/tmp/madrid-lim-pages-dist/index.html`.

## Contributing

See `CONTRIBUTING.md`. Enable the authorship hooks with `git config core.hooksPath .githooks`.

## Interaction checks

1. Confirm the hero at 1440 by 900, 1280 by 720, 390 by 844 and 320 by 568, and watch the headline cycle.
2. Hover the hero and confirm the screenshots sharpen and spread.
3. Confirm only five projects show until `show all 8 projects` is pressed, names stay on one line, and hover previews follow the pointer.
4. Open and close each project row with pointer and keyboard.
5. Scroll to runs. Confirm no MP4 loads before then, flyovers autoplay muted in view, play fully and loop, and pause when slid or scrolled away.
6. Toggle sound, step slides, hover and focus media for the description, use the `i` toggle, and download each GPX from the chip.
7. Tap the hero bubble through to the Next up dialog, try the Konami code, and confirm Escape returns focus.
8. Submit the contact form and confirm it opens a mail draft. Download the resume and grades and confirm the redactions.
9. Toggle and reload both themes. Check 200 percent zoom and reduced motion, and confirm there is no horizontal page scroll.
10. Inspect console, CSP, network requests and release asset sizes.

## Content and media

Visible project, run and contact data live in `src/content.rs`. The mounted UI is in `src/components/mod.rs`. Cutouts, post images and flyovers are in `assets/media/`, project screenshots in `assets/work/`, routes in `assets/gpx/`, redacted documents in `assets/docs/`, and Anton with its licence in `assets/fonts/`. Old files remain unreferenced and are not compiled or copied.
