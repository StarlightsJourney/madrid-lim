# Architecture

## Runtime

Trunk serves `index.html`, compiles `src/main.rs` for WebAssembly, and injects the wasm-bindgen loader. Leptos CSR mounts one semantic document workspace above a CSS-only pixel garden. The release makes no external runtime requests.

## Mounted modules

- `src/main.rs` mounts the application.
- `src/content.rs` owns six project records and contact destinations.
- `src/components/mod.rs` owns tabs, project selection, draggable window state, ticker, critters, evidence drawer, theme preference, and fixed-command terminal.
- `style/tokens.css` defines the light, dark, and garden tokens from `STYLE.md`.
- `style/main.css` defines the one-screen desktop layout, pixel garden, drag transforms, motion, and compact mobile flow.
- `assets/profile.jpg` is the optimized identity image.
- `assets/evidence/` contains real images, route video excerpts, and real poster frames.

Only `components` and `content` are referenced by the WebAssembly entry point. Older component, GL, and GPX files remain unreferenced and are not compiled or linked.

## State and boundaries

Project selection, active tab, dialog state, video activation, terminal history, newest terminal output, critter feedback, and window drag offset stay in memory. Pointer capture limits drag to blank titlebar space and viewport bounds. At 800 pixels and below, CSS and event guards remove drag offsets.

Only an explicit theme choice is stored in local storage. Terminal input is matched against a fixed command list and is never evaluated. Evidence media is absent before the drawer opens. Posters may load when it opens, while each MP4 `src` is created only after its play button is pressed. No upload, GPX parser, canvas, WebGL, unsafe HTML, source map, or remote data flow is mounted.

The evidence and terminal dialogs close with Escape or backdrop selection and restore focus to their trigger. New-tab links carry `noopener noreferrer`. The CSP permits only same-origin images, media, scripts, styles, and connections.
