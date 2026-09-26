# Architecture

## Runtime

Trunk serves `index.html`, compiles `src/main.rs` for WebAssembly, and injects the wasm-bindgen loader. Leptos CSR mounts one editorial page. The release makes no external runtime requests.

## Mounted modules

- `src/main.rs` mounts the application.
- `src/content.rs` owns project records, run posts with slides and GPX downloads, and contact destinations. Its tests check links, dashes in copy, and that every referenced asset exists.
- `src/components/mod.rs` owns the top bar, hero, marquee, work list, run reel, about, footer, theme preference, scroll spy, video observer, contact form and easter eggs.
- `style/tokens.css` defines the font face and light and dark tokens.
- `style/main.css` defines layout, motion and responsive behavior.
- `assets/media/` holds the hero and about cutouts, post photos and screenshots, and full-length flyovers with poster frames.
- `assets/work/` holds real project screenshots. `assets/gpx/` holds the downloadable routes. `assets/fonts/` holds Anton and its licence. `assets/docs/` holds the redacted resume and grades.

Only `components` and `content` are referenced by the WebAssembly entry point. Older component, GL, and GPX parser files and the old `assets/evidence/` media remain on disk, unreferenced and not copied.

## State and boundaries

The active section, hero word, bubble line, open and hovered project, show all state, slide index, description toggle, mute state, video progress, contact form fields and Next up dialog state stay in memory. Only an explicit theme choice is stored in local storage. The contact form only builds a `mailto:` URL with encoded fields; nothing is sent or stored by the site.

Window listeners (Konami keys, scroll spy), the word timer, the shared `IntersectionObserver` and the promise rejection handler are installed once and kept in thread-local slots. `play()` rejections are caught so autoplay refusals never reach the console.

## CSP

`Trunk.toml` disables file hashing so the inline loader is identical on every build. `index.html` allows that loader through two script hashes, one for the local `/` build and one for the GitHub Pages `/madrid-lim/` build. If the Trunk version or crate name changes, rebuild both and replace the hashes.
