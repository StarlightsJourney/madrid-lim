# Design system

`STYLE.md` is the binding product, layout, content, accessibility, and visual specification. This file summarizes the implementation only.

## Tokens and type

`style/tokens.css` holds the Anton `@font-face` and the light and dark palettes. Light is the default unless the system prefers dark or the user saved a choice. The theme is applied as `data-theme` on the root element. Lime (`--accent`) is always a background under dark text: nav hover, open project rows, tilted labels, highlights, the sound and info toggles when active, and the footer headline on ink.

## Page structure

A fixed translucent top bar, then a full-height hero, two crossing marquee bands, the selected work list, the runs reel, the about section, and an ink footer. Section titles, project names, post titles and the footer headline use Anton in uppercase. Body copy uses the system UI stack.

## Components

- Hero: cycling headline rebuilt per word so the letter entrance replays, one outlined letter, a cutout figure with a speech bubble, blurred real screenshots that sharpen on hover, tilted labels and an explore tab.
- Work rows: buttons with `aria-expanded` controlling `inert` regions. Panels open with a `grid-template-rows` transition. A pointer-following preview uses CSS variables set through CSSOM.
- Run posts: 9:16 media with translateX slides, slide bars, video progress, a sound toggle, an info toggle and hover descriptions. One shared `IntersectionObserver` plays each flyover at 60 percent visibility and pauses it otherwise.
- Contact: a mailto form with a live status line.
- Easter eggs: a speech bubble that cycles lines and a Next up ticket dialog opened by the bubble or the Konami code.

## Access and motion

All controls have visible focus rings and names. The skip link targets work. Mobile controls meet the 44 pixel target. Reduced motion stops word cycling, bobbing, marquee, scroll-driven effects and autoplay, and videos fall back to native controls. Content reflows without horizontal page scrolling down to 320 pixels and at 200 percent zoom.
