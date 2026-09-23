# Design system

`STYLE.md` is the binding product, layout, content, accessibility, and visual specification. This file summarizes the implementation only.

## Tokens

`style/tokens.css` contains the exact light and dark palettes. Light is the default unless the system prefers dark or the user has saved an explicit choice. Blue is the only action and selection accent. Green and yellow occur only in the traffic lights. Text uses the system UI font stack and never relies on opacity for contrast.

## One-window structure

The desktop app is one centered document window with a 44 pixel titlebar, 250 to 280 pixel identity sidebar, project workspace, and 30 pixel status line. The Projects tab uses a plain seven-row list beside one factual detail pane. About and Outside use clean reading panes, not cards. Desktop fits one viewport. Below 720 pixels, the interface becomes an edge-to-edge document with one natural vertical scroll.

## Components

Selection uses `--selection` plus a blue edge. Hairlines use `--hairline`. Links and text buttons use blue. The evidence drawer contains three real, uncropped images with captions and useful alt text. It is added only after the user opens it. The terminal accepts a fixed set of factual commands and uses `aria-live` only for its newest output.

There are no charts, maps, generated routes, fake visualizations, gradients, glows, WebGL scenes, canvas elements, stat cards, or decorative data graphics.

## Access and motion

All controls have visible focus rings and semantic names. Dialogs trap focus, close with Escape and backdrop selection, and restore focus. Mobile controls meet the 44 pixel target. Motion is limited to short color changes and disabled when reduced motion is requested. Content remains readable at 200 percent zoom and down to 320 pixels without horizontal scrolling.
