# Architecture

## Runtime

Trunk serves `index.html`, compiles `src/main.rs` for WebAssembly, and injects the wasm-bindgen loader. Leptos CSR mounts one semantic document workspace. The release has no external runtime requests.

## Mounted modules

- `src/main.rs` mounts the application.
- `src/content.rs` owns the seven project records and contact destinations.
- `src/components/mod.rs` owns the document window, tabs, project selection, evidence dialog, theme preference, and fixed-command terminal.
- `style/tokens.css` defines the exact light and dark tokens from `STYLE.md`.
- `style/main.css` defines the one-screen desktop layout and compact mobile document flow.
- `assets/evidence/` contains three optimized real images. They enter the DOM only when the evidence dialog opens.

Only `components` and `content` are referenced by the WebAssembly entry point. Older component, GL, and GPX files remain unreferenced and are not compiled or linked.

## State and boundaries

Project selection, active tab, modal state, terminal history, and newest terminal output stay in memory. Only an explicit light or dark theme choice is stored in local storage. Terminal input is matched against a fixed command list and is never evaluated. No upload, GPX parser, canvas, WebGL, unsafe HTML, source map, or remote data flow is part of the mounted app.

The evidence and terminal dialogs trap focus, close with Escape or backdrop selection, and restore focus to their trigger. New-tab links carry `noopener noreferrer`.
