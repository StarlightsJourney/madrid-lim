# Binding Design Specification

## 1. Product intent and 30-second recruiter test

This is a compact job and opportunity portfolio for Madrid Lim. It must feel like a restrained native macOS text editor, not a dashboard. Technical judgment and project work come before personality.

Within 30 seconds, a recruiter must know:
1. Who Madrid is and what opportunities he seeks.
2. Which projects best demonstrate useful technical work.
3. What Madrid personally built or decided.
4. Which outcomes and evidence are real.
5. How to contact or verify him.

The desktop first screen must answer all five without scrolling. Education and personal interests provide supporting context only.

## 2. Non-negotiable principles

- One viewport on desktop.
- Natural compact scroll only on small mobile.
- Real evidence only.
- Projects first.
- One centered app window, never a field of cards.
- High contrast, calm spacing, and obvious hierarchy.
- Every visual must clarify real information or be removed.
- Trail running is supporting texture, not the portfolio identity.
- No dashboards, stat walls, or ornamental analytics.
- No synthetic, inferred, decorative, or misleading data visuals.
- Rust, Leptos, WASM, and plain CSS remain implementation details, not hero copy.
- Keep the terminal as a secondary interaction triggered by `/`.

## 3. Background scenery

Place an original CSS and SVG pixel-art garden or meadow behind the app window. The scenery is cozy farming-game inspired, but it must not name, copy, or reproduce protected artwork from any existing game.

Build the scenery from simple, original shapes:
- Layered grass tiles with subtle color variation.
- Small plants, flowers, stones, and soft trees or shrubs around the edges.
- No parallax, no animated gradients, no glow.
- Keep all garden colors natural greens, yellows, browns, and sky tones.
- Brand and control accents inside the window stay blue.

The background is decoration only:
- Apply `aria-hidden="true"` and `pointer-events: none`.
- Never place background scenery behind reading text or essential controls.
- Keep the window surface high-contrast neutral.
- Respect `prefers-reduced-motion`: make the scenery static.

## 4. App window and draggable behavior

Use one centered application window with a titlebar, two-pane body, and bottom status line.

- Width: `min(1120px, calc(100vw - 64px))`.
- Height: `min(720px, calc(100vh - 64px))`.
- At 1280 by 720, fit within the viewport with compact spacing.
- Border: 1px solid hairline. Radius: 12px. Overflow: hidden.

On desktop, the window is draggable by its titlebar only. Drag only from the neutral titlebar area, never from traffic lights, segmented controls, or other interactive elements. Constrain the window fully within the visible viewport and update bounds on resize and zoom. Double-click the titlebar or use a compact titlebar control to reset the window to the centered position. Keyboard and non-pointer users keep the centered layout and are unaffected by drag offset.

On mobile at or below 800px, disable dragging, make the window edge-to-edge with no outer shadow, and rely on the normal centered non-drag layout.

## 5. Titlebar

A 44px titlebar contains standard traffic lights at left, a quiet document title such as `madrid.txt` near center, and tiny segmented Projects, About, and theme controls at right. Do not add a logo or full navigation bar. The titlebar is the only draggable surface on desktop.

## 6. Two-pane body

Left pane, 280px on large desktop and 248px on compact desktop:

- A small real profile image from the supplied photo: 56 to 72px, circle or rounded square, not dominant.
- Madrid Lim, a concise role line, availability, and opportunity intent.
- Location and work preference only if accurate.
- Selected external profile links: GitHub, LinkedIn, and Instagram.
- Data Science at NUS, year 3 · GPA 4.39/5.00. Keep this education line once, either in the sidebar or in About, not both.
- Never show phone number, date of birth, or student number.

Right pane:

- Split into a project list using 32 to 38 percent and selected detail using the remainder.
- List exactly six projects: Ka-teng, HillGPX, Sheng, NoSleepMenuBar, NUS Timetable Optimizer, and HejAmigo.
- Default to the strongest opportunity-relevant project supported by verified content.
- Detail shows role, problem, decisions, result, stack, and factual links.
- Keep detail concise enough to avoid desktop scrolling.

## 7. Bottom tech stack marquee

Replace the running fact status line with a continuous, slow left-to-right tech stack marquee. Use simple monochrome technology labels or locally rendered inline SVG icons for: Rust, Python, Java, Swift, PostgreSQL, PostGIS, Supabase, MapLibre, Git, and DaVinci Resolve.

- No external icon CDN.
- Duplicate the track only enough for a seamless loop.
- Screen readers receive one concise, non-animated list instead of the live track.
- Pause the animation on hover or focus.
- Honor `prefers-reduced-motion` by stopping movement.
- Maintain high contrast for all labels and icons.
- Keep a small terminal affordance fixed at the right side.

## 8. Mobile behavior

Below 800px, make the window edge-to-edge with no outer shadow. Disable dragging. Keep the titlebar sticky. Stack a compact identity and contact header, project list, selected detail, then the tech stack marquee. Permit one natural vertical scroll. Keep the marquee in document flow and prevent horizontal scrolling.

## 9. Visual reference mapping

Borrow from Adrien's site:

- Native window framing and focused document character.
- Controlled whitespace and clear type hierarchy.
- Muted compact controls.
- Subtle backdrop blur where supported.
- A soft, credible window shadow.
- Quiet but visible interaction states.

Do not copy exact identity, copy, branding, or measurements. Do not reproduce novelty that weakens project scanning, excessive translucency, decorative desktop furniture, fake operating system behavior, or a literal site clone.

## 10. Exact design tokens

### Light theme, default

```css
--page: #e8e8e6;
--window: #f7f7f5;
--titlebar: #efefed;
--sidebar: #f1f1ef;
--inset: #ffffff;
--primary: #171717;
--secondary: #666662;
--tertiary: #92928c;
--hairline: #d8d8d4;
--blue: #0a66ff;
--blue-hover: #0057db;
--focus: #006cff;
--selection: #dceaff;
--traffic-red: #ff5f57;
--traffic-yellow: #febc2e;
--traffic-green: #28c840;
--garden-grass: #8fb373;
--garden-grass-dark: #6f9661;
--garden-sky: #d7e6f0;
--garden-flower: #e8c66d;
--garden-stone: #b5b5ae;
--garden-shrub: #6b8c5a;
```

### Dark theme, optional

```css
--page: #161616;
--window: #222222;
--titlebar: #292929;
--sidebar: #292929;
--inset: #1d1d1d;
--primary: #f2f2ef;
--secondary: #b5b5ae;
--tertiary: #85857e;
--hairline: #3d3d3a;
--blue: #6aa5ff;
--blue-hover: #8ab8ff;
--focus: #7ab0ff;
--selection: #263c5c;
--garden-grass: #466040;
--garden-grass-dark: #324a2f;
--garden-sky: #1c2630;
--garden-flower: #b89a4d;
--garden-stone: #6e6e68;
--garden-shrub: #3e5634;
```

Body text must meet WCAG AA at 4.5:1. Large text, essential controls, and focus indicators must meet 3:1. Use primary or secondary for project content. Reserve tertiary for nonessential metadata. Never reduce text contrast with opacity. Green and yellow appear only in standard traffic lights. Blue is the sole action and selection accent.

## 11. Typography

Use system UI only:

```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
```

- Name: 34 to 44px, weight 650 to 700, line height 1.05.
- Role: 16 to 18px, weight 450 to 550, line height 1.35.
- Section label: 11 to 12px, weight 600, line height 1.3.
- Project rows: 14 to 15px, weight 500 to 600, line height 1.35.
- Detail title: 22 to 28px, weight 650, line height 1.15.
- Detail body: 13 to 15px, weight 400, line height 1.5.
- Metadata and status: 11 to 12px, line height 1.35.
- No text below 11px. Avoid all caps except short labels.
- Use tabular numbers for time, distance, GPA, and elevation gain.
- Never use a giant display headline.

## 12. Components and interactions

### Pixel critters

Add one or two tiny original pixel critters around the outside background, such as a walking chicken and a butterfly or firefly. They must never cross the app window, never play sound, and pause when hidden or when reduced motion is requested. Motion should be slow and subtle, around 18 to 30 seconds per cycle. A click or tap may show one concise bubble such as `found me.` and then dismiss quickly.

### Project list and detail

- Use a simple vertical list, not cards. Rows are at least 36px high.
- Each row has a project name and one short category or outcome.
- Selection uses the selection fill plus a blue edge or dot. Hover stays subtle.
- Detail contains a short summary, Problem, Decisions, Result, quiet stack metadata, and verified links.
- Do not invent metrics or outcomes.

### Controls, links, and contact

- Segmented controls are 24 to 28px high with clear selected states.
- Use native buttons with accessible names.
- Links use blue text or a blue underline on hover and focus.
- Contact links are GitHub, LinkedIn, and Instagram. External links disclose new-tab behavior when applicable.

### Evidence panel

Open from `View evidence`. Use a drawer on desktop and a full-height modal sheet on mobile. Include a heading, close button, and brief factual captions. Trap focus while open and restore it on close.

Keep the existing three real images: the running portrait, the 202.54 km route screenshot, and the Rinjani screenshot. Add up to three optimized, short, muted route flyover videos. Do not autoplay videos with sound. Use a real poster frame from each clip. Load video bytes only when the panel opens and the user explicitly presses play. Label generic route clips without inventing specific achievements if uncertain. The pane title and lead are enough context; remove any `supporting proof` label.

### Terminal

- `/` opens a clean overlay unless focus is in an input. Escape closes it.
- Allow only fixed factual commands such as `help`, `projects`, `about`, `contact`, `clear`, and `close`.
- Never imitate a security shell or expose environment details.
- Make it fully usable by keyboard and screen reader.

### Motion

Use restrained motion:
- Window entrance: 180ms scale and opacity.
- Project detail crossfade and slide: 140ms.
- Selection feedback: spring-like but non-bouncy, 160ms.
- Controls: 120ms color or opacity.
- Tech stack marquee: 28 to 36s linear, left to right.
- Pixel critters: 18 to 30s, subtle travel.

No parallax, no large transforms, no glow. Reduced motion must disable entrance transforms, the marquee, critter travel, and the detail slide. State changes remain immediate but visible.

## 13. Content hierarchy and approved tone

Order content as follows:
1. Name, role, and availability.
2. Projects and technical decisions.
3. Contact and verification links.
4. Education and academic evidence.
5. Personal endurance evidence.

Use concise, direct, factual sentences and concrete verbs such as built, designed, measured, tested, and shipped. State personal contribution precisely. Use sentence case. No em dash or en dash characters. No AI wording in visible copy. No buzzwords, unsupported claims, vague superlatives, overclaiming, fake testimonials, or fake client language.

## 14. Real media policy

- Use the real profile photo in the sidebar.
- Use the real running portrait, the 202.54 km route screenshot, and the Rinjani screenshot only inside the user-opened evidence panel.
- Never use personal media as a hero, backdrop, or full-bleed image.
- Preserve aspect ratios and factual screenshot labels. Use short factual captions.
- Valid evidence includes the 202.54 km route screenshot and Rinjani screenshot showing 61.27 km, 19:47:29, and 5,287 m gain.
- Do not create placeholders.
- Videos are optional muted previews after optimization. Load no video bytes until the panel opens and playback is requested.
- GPX routes and maps may use only actual user-supplied files after privacy review for home, start, finish, and repeated-location exposure.
- If provenance or privacy is uncertain, omit the media.

## 15. Explicit delete and remove list

Remove:

- WebGL terrain and globe.
- Long scroll sections.
- Altimeter and dock.
- Fake route SVG and route outlines.
- Illustrative elevation profiles and peak chart.
- Generated photo placeholders.
- Project-window grid.
- Giant headline.
- Green and orange theme.
- Decorative stats charts, fake maps, 3D terrain, and any misleading visualization.
- Vertical SG from the project list.
- The running fact status line at the bottom.
- Repeated education blocks. Keep the year and GPA in one place only.
- Any `supporting proof` label.

## 16. Accessibility and responsive acceptance criteria

- Use semantic landmarks, headings, lists, buttons, links, and dialog roles.
- Every control is keyboard reachable, operable, named, and visibly focused.
- Focus never sits behind sticky UI. Selection is conveyed by more than color.
- Theme follows system preference and persists only as a user preference.
- Both themes meet stated contrast thresholds.
- Text remains usable at 200 percent zoom and reflows without horizontal scroll at 320px.
- Mobile touch targets are at least 44 by 44px. Desktop compact controls may be 28px high when safely spaced.
- Images have useful alt text or empty alt text when captions duplicate them.
- Dialogs announce name and state. Reduced motion is honored.
- Content remains understandable when CSS, media, or WebGL is unavailable.
- Drag bounds keep the window fully inside the viewport at 1280 by 720 and 1440 by 900.
- Drag reset by double-clicking the titlebar or a compact control returns the window to center.
- At 320px and 390px widths, the layout is edge-to-edge with no drag and no horizontal scroll.
- At 800px and below, dragging is disabled and the window becomes edge-to-edge.
- Day and dark garden values pass contrast against the window surface, and all text inside the window meets WCAG contrast.
- Video elements load lazily, use `preload="none"`, and never autoplay with sound.
- The marquee is invisible to screen readers and pauses on hover, focus, and reduced motion.
- Pixel critters never overlap the window or interfere with pointer targets.
- Initial media payload stays below 500 KB including the profile image, and total evidence videos stay under 6 MB combined with each video no larger than 2 MB.

## 17. Engineering and security constraints

- Keep Rust, Leptos CSR, WASM, Trunk, and plain CSS.
- Make no external runtime requests.
- Use system fonts. Initial media payload stays below 500 KB excluding WASM.
- Keep each optimized image below 300 KB where legibility permits.
- Do not preload evidence images or video.
- Video is lazy, muted by default, and loaded only after explicit action. Poster frames must come from actual video. Use `preload="none"`.
- Each optimized video should be 2 MB or smaller.
- Keep profile image and evidence media same-origin.
- Apply a restrictive CSP compatible with the WASM build. Allow `media-src 'self'` and same-origin connections only unless a documented feature requires otherwise.
- New-tab links use `rel="noopener noreferrer"`.
- Never render unsafe HTML, bypass escaping, or evaluate terminal input as code. Validate commands against a fixed allowlist.
- If GPX import remains, process locally, never upload or persist it, cap files at 10 MB, and reject invalid input without panicking.
- Pin third-party workflow actions to immutable commit SHAs.
- Keep dependencies and Rust feature flags minimal and reviewed.
- Never expose source maps, secrets, private identifiers, phone number, date of birth, or student number.
