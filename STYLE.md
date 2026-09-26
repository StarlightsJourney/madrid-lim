# Binding Design Specification

## 1. Product intent and 30-second recruiter test

This is a job and opportunity portfolio for Madrid Lim, presented as a bold editorial page inspired by poster-style studio portfolios. It should feel confident and playful, but every section must still carry real information.

Within 30 seconds, a visitor must know:
1. Who Madrid is and what opportunities he seeks.
2. Which projects best demonstrate useful technical work.
3. What Madrid personally built or decided on each project.
4. Which outcomes and evidence are real.
5. How to contact or verify him.

## 2. Principles

- One scrolling page with anchors for top, work, runs, about and contact.
- Projects come before running. Running is personality and evidence, not the main identity.
- Real media only. No placeholders, generated imagery, fake maps or inferred data.
- Big type and motion must never hide content, trap focus or block reading.
- Do not copy another site's identity, copy, artwork, code or measurements. Borrow structure and energy only.

## 3. Layout

### Top bar
Fixed, 64px, translucent with backdrop blur. Left: `ML` mark and location. Center: pill navigation for work, runs, about and contact, with the section in view shown as a filled pill. Right: a resume download pill, GitHub, LinkedIn and Instagram icons, and the theme toggle. Below 900px, hide the location, resume pill and social icons. Below 380px, hide the mark.

### Hero
- A giant Anton headline cycles through short role words every 2.6s with a staggered letter entrance. One letter per word is drawn as an outline.
- A real background-removed photo of Madrid stands in the center, overlapping the headline, with a `hi, i'm Madrid.` speech bubble button.
- Real project screenshots fan out behind the figure, softly blurred. They sharpen and spread on hover.
- Two tilted lime labels state education and focus.
- Where supported, the headline, figure and screenshots respond to scroll through CSS scroll-driven animation only.

### Marquee
Two crossing bands, lime with project names and ink with the tech stack, moving in opposite directions. Decorative bands are `aria-hidden`; screen readers get one static technology list. Pause on hover.

### Selected work
- A numbered list of large project names, each on one line with an ellipsis when too long, plus category, status and a plus toggle.
- Show the first five projects. A `show all 8 projects` row reveals the rest and can collapse them again.
- Hovering a row inverts it, and a preview (real screenshot or a lime type card with the name on one line) follows the pointer on hover-capable devices wider than 900px.
- Activating a row expands an accessible region with summary, role, problem, what Madrid built, stack chips and links. Private repositories are described and marked, never linked.
- Projects: Linguini, HillGPX, Ka-teng, Sheng, CivicTwin, NUS Timetable Optimizer, HejAmigo, NoSleepMenuBar. State team size and personal contribution precisely.

### Outside the screen (runs)
- A horizontal reel of post cards styled like social posts: header with avatar, place and date; a 9:16 media area; title and stats on one line.
- Posts: North Lombok (Rinjani), CULTRA 60 km in Cameron Highlands, 202.54 km around Singapore, and BUS Backyard Ultra (Aug 7, 2026, 83.52 km, third).
- Media slides pair the full-length COROS route flyover with the matching screenshot and photos. Slide bars, previous and next buttons, and a live slide count are provided.
- Flyovers autoplay muted and loop only while at least 60 percent visible, and pause when scrolled or slid away. A sound toggle unmutes on request. With reduced motion, videos show native controls and never autoplay.
- The post note and slide description appear only on hover, keyboard focus, or the `i` toggle on touch devices. They must match what the media shows.
- A lime GPX chip sits at the bottom left of the media for posts with a route. It downloads the original file the user supplied, same-origin, and shows the file size.

### About
A large statement with lime highlighted phrases, a cutout photo with a sticker, and three columns: education, experience, and races with the next race flagged. Keep the education line once: NUS B.Sc. Data Science, year 3, GPA 4.39/5.00.

### Documents
The resume and NUS grades are offered as PDF downloads in the top bar (resume) and the contact section. Published copies are flattened to images with the phone number, student number and date of birth removed. Never publish the original files.

### Footer and contact
Ink background with a giant lime `say hi`, then a `Write to me` section with the NUS email (e1398088@u.nus.edu) as a same-tab mail link and a short form (name, topic, message). Submitting builds a `mailto:` link with an encoded subject and body and opens the visitor's email app. No form data leaves the browser any other way. Below: pill links to GitHub, LinkedIn and Instagram, a subtle Konami hint and back to top.

### Easter eggs
- Tapping the hero speech bubble cycles short lines about what is next, and the last tap opens the Next up dialog.
- The Konami code opens the same dialog. It shows three tickets: the BTS170 race bib (Bromo Tengger Semeru, East Java, Nov 7 to 8, 2026), the Sheng public launch (end of 2026), and a boarding pass to India and Africa. The dialog traps focus, closes with Escape or the backdrop, and returns focus.
- A short console greeting for developers.
- Easter egg content must stay factual.

## 4. Tokens

```css
/* light, default */
--bg: #f6f5f1; --card: #ffffff; --ink: #111110; --ink-2: #55544f;
--line: #dcdad3; --accent: #d4f53c; --accent-ink: #111110; --focus: #2447e6;
/* dark */
--bg: #0d0d0c; --card: #181816; --ink: #f3f2ec; --ink-2: #b3b1a8;
--line: #2e2d2a; --accent: #d4f53c; --accent-ink: #0d0d0c; --focus: #9db4ff;
```

Lime is used as a background under dark text, never as text on the light background. Body text meets WCAG AA 4.5:1 and large text, controls and focus rings meet 3:1 in both themes. Never reduce text contrast with opacity.

## 5. Typography

- Display: self-hosted Anton (SIL OFL, `assets/fonts/`), uppercase, for the hero, section titles, project names, post titles and the footer.
- Body: `-apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`.
- Monospace for the Konami hint only.
- No body text below 11px. Use tabular numbers for time, distance and elevation.

## 6. Motion

Letter entrances 640ms, row and panel transitions 350 to 500ms, marquee 48 to 56s linear, hero bob 6s. Reduced motion stops the marquee, word cycling, bobbing, confetti, scroll-driven effects and video autoplay, and makes transitions immediate.

## 7. Content and tone

Concise, factual, sentence case in source. No em or en dash characters, AI wording, buzzwords, invented metrics or fake testimonials. Never show a phone number, date of birth or student number.

## 8. Media policy

- Hero and about cutouts come from the user's own photos using on-device background removal.
- Project screenshots are captured from the real running apps.
- Route flyovers are the full COROS exports, re-encoded to 432 by 960 H.264 with faststart, `preload="none"`, and real poster frames. No video bytes load until a post scrolls into view.
- Initial page media stays under 500 KB excluding WebAssembly. Images are WebP where possible.
- GPX files are published as supplied at the owner's explicit request. They include full tracks and sensor data.

## 9. Accessibility and responsive acceptance

- Semantic landmarks, headings, lists, buttons, links and dialog roles. A skip link goes to work.
- Every control is keyboard reachable, named and visibly focused. Expanded state uses `aria-expanded`; collapsed panels are `inert`.
- No horizontal page scroll at 320, 390, 1280 and 1440 widths or at 200 percent zoom. The reel is the only horizontal scroller and is keyboard focusable.
- Touch targets at least 44 by 44px on mobile.
- Theme follows system preference; only an explicit choice persists in local storage.

## 10. Engineering and security

- Rust, Leptos CSR, WebAssembly, Trunk and plain CSS. No JavaScript frameworks, TypeScript, Tailwind, WebGL or canvas.
- No external runtime requests. Fonts, images, video and GPX are same-origin.
- Restrictive CSP: `default-src 'self'`, `font-src 'self'`, `media-src 'self'`, `style-src 'self'`, and `script-src 'self' 'wasm-unsafe-eval'` plus the hashes of the Trunk loader for `/` and `/madrid-lim/`. Trunk file hashing is off so the loader and its hash stay stable.
- Styles set at runtime go through CSSOM, never inline style attributes.
- New-tab links use `rel="noopener noreferrer"`. Never render unsafe HTML.
- Browser APIs are called fallibly. Pin third-party workflow actions to commit SHAs.
