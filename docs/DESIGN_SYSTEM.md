# Design system

`STYLE.md` is the binding product, layout, content, accessibility, and visual specification. This file summarizes the implementation only.

## Tokens and scenery

`style/tokens.css` contains the light, dark, and garden palettes. Light is the default unless the system prefers dark or the user saved a choice. Blue is the sole action and selection accent inside the window. The outer garden uses original CSS pixel tiles, shrubs, trees, flowers, stones, and two small interactive critters. It stays behind the neutral window and is hidden on mobile.

## One-window structure

The desktop app is one centered, draggable document window with a 44 pixel titlebar, 248 to 280 pixel identity pane, project workspace, and 34 pixel ticker. The identity uses a small real profile crop. Projects uses a plain six-row list beside one factual detail pane. About contains the only year and GPA line. Outside uses a clean reading pane. At 800 pixels and below, the interface becomes an edge-to-edge document with one natural vertical scroll and no drag transform.

## Components

Selection uses `--selection` plus a blue edge. The tech ticker uses one screen-reader list and two `aria-hidden` visual tracks. Locally rendered geometric glyphs avoid icon dependencies. The evidence drawer contains three real images and three poster-first real route videos. MP4 sources are created only after explicit play. The terminal accepts fixed factual commands and uses `aria-live` only for its newest output.

There are no charts, generated routes, fake visualizations, WebGL scenes, canvas elements, stat cards, or decorative data graphics inside the window.

## Access and motion

All controls have visible focus rings and semantic names. Dialogs close with Escape and backdrop selection and restore focus. Mobile controls meet the 44 pixel target. The window enters in 180ms, project detail changes in 140ms, selection in 160ms, controls in 120ms, and the ticker moves left-to-right over 32 seconds. Reduced motion removes transforms, travel, ticker duplication, and detail movement. Content remains readable at 200 percent zoom and down to 320 pixels without horizontal scrolling.
