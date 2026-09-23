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

## 3. Exact single-screen information architecture

### App window

Use one centered application window with a titlebar, two-pane body, and bottom status line.

- Width: `min(1120px, calc(100vw - 64px))`.
- Height: `min(720px, calc(100vh - 64px))`.
- At 1280 by 720, fit within the viewport with compact spacing.
- Border: 1px solid hairline. Radius: 12px. Overflow: hidden.

### Titlebar

A 44px titlebar contains standard traffic lights at left, a quiet document title such as `madrid.txt` near center, and tiny segmented Projects, About, and theme controls at right. Do not add a logo or full navigation bar.

### Two-pane body

Left pane, 280px on large desktop and 248px on compact desktop:

- Madrid Lim, a concise role line, availability, and opportunity intent.
- Location and work preference only if accurate.
- Selected external profile links: GitHub, LinkedIn, and Instagram.
- NUS Data Science, year 3.
- Optional academic proof: GPA 4.39/5, best semester 4.80, A+ in CS1010S.
- Never show phone number, date of birth, or student number.

Right pane:

- Split into a project list using 32 to 38 percent and selected detail using the remainder.
- List Ka-teng, HillGPX, Sheng, NoSleepMenuBar, Vertical SG, NUS Timetable Optimizer, and HejAmigo.
- Default to the strongest opportunity-relevant project supported by verified content.
- Detail shows role, problem, decisions, result, stack, and factual links.
- Keep detail concise enough to avoid desktop scrolling.

### Bottom status line

Use a 28px line with one personal fact at left and `/ terminal` at right. A suitable fact is 202.54 km around Singapore in 46:46:23, raising $850. Keep this line secondary and static by default.

### Mobile behavior

Below 800px, make the window edge-to-edge with no outer shadow. Keep the titlebar sticky. Stack a compact identity and contact header, project list, selected detail, then status line. Permit one natural vertical scroll. Keep status in document flow and prevent horizontal scrolling.

## 4. Visual reference mapping

Borrow from Adrien's site:

- Native window framing and focused document character.
- Controlled whitespace and clear type hierarchy.
- Muted compact controls.
- Subtle backdrop blur where supported.
- A soft, credible window shadow.
- Quiet but visible interaction states.

Do not copy exact identity, copy, branding, or measurements. Do not reproduce novelty that weakens project scanning, excessive translucency, decorative desktop furniture, fake operating system behavior, or a literal site clone.

## 5. Exact design tokens

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
```

Body text must meet WCAG AA at 4.5:1. Large text, essential controls, and focus indicators must meet 3:1. Use primary or secondary for project content. Reserve tertiary for nonessential metadata. Never reduce text contrast with opacity. Green and yellow appear only in standard traffic lights. Blue is the sole action and selection accent.

## 6. Typography

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

## 7. Components and interactions

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

### Terminal

- `/` opens a clean overlay unless focus is in an input. Escape closes it.
- Allow only fixed factual commands such as `help`, `projects`, `about`, `contact`, `clear`, and `close`.
- Never imitate a security shell or expose environment details.
- Make it fully usable by keyboard and screen reader.

### Keyboard and motion

Arrow keys move through the focused project list. Enter or Space selects. Tab order follows visual order. Escape closes the top overlay. Focus rings are mandatory. Transitions last 120 to 180ms and affect color, opacity, or small transforms only. Reduced motion removes transforms and makes state changes immediate.

## 8. Content hierarchy and approved tone

Order content as follows:
1. Name, role, and availability.
2. Projects and technical decisions.
3. Contact and verification links.
4. Education and academic evidence.
5. Personal endurance evidence.

Use concise, direct, factual sentences and concrete verbs such as built, designed, measured, tested, and shipped. State personal contribution precisely. Use sentence case. No em dash or en dash characters. No AI wording in visible copy. No buzzwords, unsupported claims, vague superlatives, overclaiming, fake testimonials, or fake client language.

## 9. Real media policy

- Use the real running portrait and COROS screenshots only inside a user-opened `Outside the screen` evidence panel.
- Never use personal media as a hero, backdrop, or full-bleed image.
- Preserve aspect ratios and factual screenshot labels. Use short factual captions.
- Valid evidence includes the 202.54 km route screenshot and Rinjani screenshot showing 61.27 km, 19:47:29, and 5,287 m gain.
- Do not create placeholders.
- Videos are optional muted previews after optimization. Load no video bytes until the panel opens and playback is requested. Keep portrait video portrait.
- GPX routes and maps may use only actual user-supplied files after privacy review for home, start, finish, and repeated-location exposure.
- If provenance or privacy is uncertain, omit the media.

## 10. Explicit delete and remove list

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

## 11. Accessibility and responsive acceptance criteria

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

## 12. Visual acceptance checklist

### 1440 by 900

- One centered window fits fully with at least 32px outer margin.
- All project names and the default detail are visible without scrolling.
- Shadow is subtle and not halo-like.

### 1280 by 720

- The full window fits with no desktop page scroll or status overlap.
- Name is no larger than 40px.
- Project list and detail remain simultaneously usable.

### 390 by 844

- Window is edge-to-edge with one natural vertical scroll and no horizontal scroll.
- Identity, contact, list, detail, and status follow the specified order.
- Evidence sheet and terminal fit, remain operable, and are dismissible.

### Day and dark themes

- Pane boundaries, selection, and focus remain clear.
- All text and controls meet contrast requirements.
- No green or orange brand accent appears. Media is not recolored.

### 200 percent zoom

- Core content reflows with no clipped text or inaccessible control.
- Modal, terminal, and project selection remain operable.
- Reading never requires horizontal scrolling.

## 13. Engineering and security constraints

- Keep Rust, Leptos CSR, WASM, Trunk, and plain CSS.
- Make no external runtime requests.
- Use system fonts. Initial media payload stays below 500 KB excluding WASM.
- Keep each optimized image below 300 KB where legibility permits.
- Do not preload evidence images or video.
- Video is lazy, muted by default, and loaded only after explicit action. Poster frames must come from actual video.
- Apply a restrictive CSP compatible with the WASM build. Allow same-origin connections only unless a documented feature requires otherwise.
- New-tab links use `rel="noopener noreferrer"`.
- Never render unsafe HTML, bypass escaping, or evaluate terminal input as code. Validate commands against a fixed allowlist.
- If GPX import remains, process locally, never upload or persist it, cap files at 10 MB, and reject invalid input without panicking.
- Pin third-party workflow actions to immutable commit SHAs.
- Keep dependencies and Rust feature flags minimal and reviewed.
- Never expose source maps, secrets, private identifiers, phone number, date of birth, or student number.
