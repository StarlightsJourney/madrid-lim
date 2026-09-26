use std::cell::RefCell;

use leptos::html::{Button, Ol, Ul, Video};
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlElement, HtmlMediaElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit, KeyboardEvent, PointerEvent, ScrollBehavior, ScrollToOptions,
    Storage,
};

use crate::content::{CONTACTS, DOCUMENTS, EMAIL, PROJECTS, Project, RUNS, RunPost, Slide};

type KeyListener = Closure<dyn FnMut(KeyboardEvent)>;
type ScrollListener = Closure<dyn FnMut(web_sys::Event)>;
type ObserverCallback = Closure<dyn FnMut(js_sys::Array, IntersectionObserver)>;
type PromiseHandler = Closure<dyn FnMut(JsValue)>;

thread_local! {
    static GLOBAL_KEY_LISTENER: RefCell<Option<KeyListener>> = const { RefCell::new(None) };
    static SCROLL_LISTENER: RefCell<Option<ScrollListener>> = const { RefCell::new(None) };
    static VIDEO_OBSERVER: RefCell<Option<(IntersectionObserver, ObserverCallback)>> = const { RefCell::new(None) };
    static IGNORE_REJECTION: RefCell<Option<PromiseHandler>> = const { RefCell::new(None) };
    static WORD_TIMER: RefCell<Option<Closure<dyn FnMut()>>> = const { RefCell::new(None) };
}

const HERO_WORDS: [(&str, usize); 4] = [
    ("builder", 0),
    ("data science", 5),
    ("maps and gpx", 0),
    ("ultra runner", 7),
];

const TECHNOLOGIES: [&str; 10] = [
    "Rust",
    "Python",
    "Java",
    "Swift",
    "TypeScript",
    "PostgreSQL",
    "PostGIS",
    "Supabase",
    "MapLibre",
    "DaVinci Resolve",
];

const SECTIONS: [(&str, &str); 4] = [
    ("work", "work"),
    ("runs", "runs"),
    ("about", "about"),
    ("contact", "contact"),
];

const VISIBLE_PROJECTS: usize = 5;

const BUBBLE_LINES: [&str; 5] = [
    "hi, i'm Madrid.",
    "next race: BTS170 in November.",
    "Sheng goes public this year.",
    "India and Africa are on my map.",
    "one more tap...",
];

const KONAMI: [&str; 10] = [
    "ArrowUp",
    "ArrowUp",
    "ArrowDown",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "ArrowLeft",
    "ArrowRight",
    "b",
    "a",
];

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = signal(initial_theme());
    let (active, set_active) = signal("");
    let (egg_open, set_egg_open) = signal(false);
    let (egg_return, set_egg_return) = signal(None::<HtmlElement>);
    let open_egg = move || {
        let focused = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.active_element())
            .and_then(|element| element.dyn_into::<HtmlElement>().ok());
        set_egg_return.set(focused);
        set_egg_open.set(true);
    };
    let close_egg = move || {
        set_egg_open.set(false);
        if let Some(element) = egg_return.get_untracked() {
            let _ = element.focus();
        }
    };

    install_easter_eggs(egg_open, open_egg, close_egg);
    install_scroll_spy(set_active);
    web_sys::console::log_1(&JsValue::from_str(
        "hi, fellow dev. try the konami code, or tap the speech bubble. say hi: e1398088@u.nus.edu",
    ));

    Effect::new(move |_| {
        let theme = theme.get();
        let document = web_sys::window().and_then(|window| window.document());
        if let Some(root) = document
            .as_ref()
            .and_then(|document| document.document_element())
        {
            let _ = root.set_attribute("data-theme", theme);
        }
        if let Some(meta) = document.and_then(|document| document.get_element_by_id("theme-color"))
        {
            let color = if theme == "dark" {
                "#0d0d0c"
            } else {
                "#f6f5f1"
            };
            let _ = meta.set_attribute("content", color);
        }
    });
    let toggle_theme = move || {
        let next = if theme.get_untracked() == "light" {
            "dark"
        } else {
            "light"
        };
        set_theme.set(next);
        store_theme(next);
    };
    view! {
        <a class="skip-link" href="#work">"skip to work"</a>
        <TopBar active theme toggle_theme />
        <main>
            <Hero open_egg />
            <Marquee />
            <Work />
            <Runs />
            <About />
        </main>
        <Footer open_egg />
        <Show when=move || egg_open.get()>
            <NextUp close=close_egg />
        </Show>
    }
}

#[component]
fn TopBar(
    active: ReadSignal<&'static str>,
    theme: ReadSignal<&'static str>,
    toggle_theme: impl Fn() + Copy + 'static,
) -> impl IntoView {
    view! {
        <header class="topbar">
            <a class="brand" href="#top" aria-label="Madrid Lim, back to top">
                <span class="brand-mark" aria-hidden="true">"ML"</span>
                <span class="brand-place"><PinIcon />"Singapore"</span>
            </a>
            <nav class="pill-nav" aria-label="Sections">
                {SECTIONS.into_iter().map(|(id, label)| view! {
                    <a href=format!("#{id}") class:active=move || active.get() == id aria-current=move || (active.get() == id).then_some("true")>{label}</a>
                }).collect_view()}
            </nav>
            <div class="topbar-actions">
                <a class="resume-pill" href=DOCUMENTS[0].href download="madrid-lim-resume.pdf"><span>"resume"</span><i aria-hidden="true">"\u{2193}"</i></a>
                <SocialLinks />
                <button class="theme-toggle" aria-label=move || if theme.get() == "light" { "use dark theme" } else { "use light theme" } on:click=move |_| toggle_theme()>
                    <span aria-hidden="true">{move || if theme.get() == "light" { "\u{25D0}" } else { "\u{25D1}" }}</span>
                </button>
            </div>
        </header>
    }
}

#[component]
fn SocialLinks() -> impl IntoView {
    view! {
        <ul class="social-links">
            {CONTACTS.into_iter().map(|(label, href)| view! {
                <li><a href=href target="_blank" rel="noopener noreferrer" aria-label=format!("{label}, opens in a new tab")><SocialIcon name=label /></a></li>
            }).collect_view()}
        </ul>
    }
}

#[component]
fn SocialIcon(name: &'static str) -> impl IntoView {
    let path = match name {
        "GitHub" => {
            "M12 .5a11.5 11.5 0 0 0-3.64 22.41c.58.1.79-.25.79-.56v-2c-3.2.7-3.88-1.37-3.88-1.37-.52-1.33-1.28-1.69-1.28-1.69-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.7 1.26 3.36.96.1-.75.4-1.26.73-1.55-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.28 1.19-3.09-.12-.29-.52-1.46.11-3.05 0 0 .97-.31 3.17 1.18a11 11 0 0 1 5.77 0c2.2-1.49 3.17-1.18 3.17-1.18.63 1.59.23 2.76.11 3.05.74.81 1.19 1.83 1.19 3.09 0 4.42-2.69 5.39-5.26 5.68.41.36.78 1.06.78 2.14v3.17c0 .31.21.67.8.56A11.5 11.5 0 0 0 12 .5Z"
        }
        "LinkedIn" => {
            "M4.98 3.5a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5ZM3 9.75h4v11.5H3V9.75Zm6.5 0h3.83v1.57h.06c.53-1 1.84-2.07 3.79-2.07 4.05 0 4.8 2.67 4.8 6.13v5.87h-4v-5.2c0-1.24-.02-2.84-1.73-2.84-1.73 0-2 1.35-2 2.75v5.29h-4V9.75Z"
        }
        _ => {
            "M12 7a5 5 0 1 0 0 10 5 5 0 0 0 0-10Zm0 8.2a3.2 3.2 0 1 1 0-6.4 3.2 3.2 0 0 1 0 6.4ZM17.3 5.5a1.2 1.2 0 1 0 0 2.4 1.2 1.2 0 0 0 0-2.4ZM12 2.5c-2.6 0-2.9 0-3.9.06C4.6 2.72 2.72 4.6 2.56 8.1 2.5 9.1 2.5 9.4 2.5 12s0 2.9.06 3.9c.16 3.5 2.04 5.38 5.54 5.54 1 .06 1.3.06 3.9.06s2.9 0 3.9-.06c3.5-.16 5.38-2.04 5.54-5.54.06-1 .06-1.3.06-3.9s0-2.9-.06-3.9c-.16-3.5-2.04-5.38-5.54-5.54-1-.06-1.3-.06-3.9-.06Zm0 1.7c2.55 0 2.85 0 3.86.06 2.58.12 3.8 1.35 3.92 3.92.05 1 .06 1.31.06 3.86s0 2.85-.06 3.86c-.12 2.57-1.33 3.8-3.92 3.92-1 .05-1.31.06-3.86.06s-2.85 0-3.86-.06c-2.59-.12-3.8-1.35-3.92-3.92-.05-1-.06-1.31-.06-3.86s0-2.85.06-3.86C4.34 5.6 5.55 4.38 8.14 4.26c1-.05 1.31-.06 3.86-.06Z"
        }
    };
    view! { <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true" focusable="false"><path fill="currentColor" d=path /></svg> }
}

#[component]
fn PinIcon() -> impl IntoView {
    view! { <svg viewBox="0 0 24 24" width="12" height="12" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 2a7 7 0 0 0-7 7c0 5.25 7 13 7 13s7-7.75 7-13a7 7 0 0 0-7-7Zm0 9.5A2.5 2.5 0 1 1 12 6.5a2.5 2.5 0 0 1 0 5Z" /></svg> }
}

#[component]
fn Hero(open_egg: impl Fn() + Copy + 'static) -> impl IntoView {
    let (word, set_word) = signal(0usize);
    let (line, set_line) = signal(0usize);
    if !reduced_motion() {
        install_word_timer(set_word);
    }
    view! {
        <section id="top" class="hero" aria-labelledby="hero-title">
            <h1 id="hero-title" class="hero-title">
                <span class="sr-only">"Madrid Lim. Builder, data science student, maps and GPX tinkerer, ultra runner."</span>
                {move || {
                    let (text, outlined) = HERO_WORDS[word.get()];
                    view! {
                        <span class="hero-word" aria-hidden="true">
                            {text.split(' ').enumerate().map(|(part_index, part)| {
                                let offset = text.split(' ').take(part_index).map(|p| p.len() + 1).sum::<usize>();
                                view! {
                                    <span class="hero-line">
                                        {part.chars().enumerate().map(|(index, character)| view! {
                                            <span class="ch" class:outlined=offset + index == outlined>{character.to_string()}</span>
                                        }).collect_view()}
                                    </span>
                                }
                            }).collect_view()}
                        </span>
                    }
                }}
            </h1>
            <div class="hero-stage" aria-hidden="true">
                <img class="shot shot-1" src="assets/work/linguini-landing.webp" alt="" decoding="async" />
                <img class="shot shot-2" src="assets/work/hillgpx.webp" alt="" decoding="async" />
                <img class="shot shot-3 phone" src="assets/work/linguini-app.webp" alt="" decoding="async" />
                <img class="shot shot-4" src="assets/work/kateng.webp" alt="" decoding="async" />
                <img class="shot shot-5 phone" src="assets/media/rinjani.webp" alt="" decoding="async" />
            </div>
            <figure class="hero-figure">
                <img src="assets/media/hero.webp" width="720" height="1406" alt="Madrid Lim smiling and making a peace sign" fetchpriority="high" decoding="async" />
                <figcaption>
                    <button class="hero-bubble" aria-label=move || format!("{} Tap for more.", BUBBLE_LINES[line.get()]) on:click=move |_| {
                        if line.get_untracked() + 1 == BUBBLE_LINES.len() {
                            set_line.set(0);
                            open_egg();
                        } else {
                            set_line.update(|value| *value += 1);
                        }
                    }>
                        {move || view! { <span class="bubble-text">{BUBBLE_LINES[line.get()]}</span> }}
                    </button>
                </figcaption>
            </figure>
            <p class="tilt tilt-left">"Data Science @ NUS"</p>
            <p class="tilt tilt-right">"builds tools / runs ultras"</p>
        </section>
    }
}

#[component]
fn Marquee() -> impl IntoView {
    let names = || {
        (0..4)
            .flat_map(|_| PROJECTS.iter().map(|project| project.name))
            .map(|name| view! { <span>{name}</span><i>"\u{2726}"</i> })
            .collect_view()
    };
    let tools = || {
        (0..4)
            .flat_map(|_| TECHNOLOGIES)
            .map(|name| view! { <span>{name}</span><i>"\u{2726}"</i> })
            .collect_view()
    };
    view! {
        <div class="marquee-block">
            <ul class="sr-only" aria-label="Technology stack">{TECHNOLOGIES.into_iter().map(|name| view! { <li>{name}</li> }).collect_view()}</ul>
            <div class="band band-accent" aria-hidden="true"><div class="band-track">{names()}</div><div class="band-track">{names()}</div></div>
            <div class="band band-ink" aria-hidden="true"><div class="band-track reverse">{tools()}</div><div class="band-track reverse">{tools()}</div></div>
        </div>
    }
}

#[component]
fn Work() -> impl IntoView {
    let (open, set_open) = signal(None::<usize>);
    let (hovered, set_hovered) = signal(None::<usize>);
    let (show_all, set_show_all) = signal(false);
    let list = NodeRef::<Ol>::new();
    view! {
        <section id="work" class="section work" aria-labelledby="work-title">
            <header class="section-head">
                <h2 id="work-title" class="section-title">"Selected work"<sup>{format!("({:02})", PROJECTS.len())}</sup></h2>
                <p class="section-lead">"Tools I designed, built and shipped. Open a project for my role, the problem and what I built. Private repositories are described, not linked."</p>
            </header>
            <ol node_ref=list class="work-list"
                on:pointermove=move |event: PointerEvent| {
                    if let Some(element) = list.get() {
                        let rect = element.get_bounding_client_rect();
                        let html: &HtmlElement = &element;
                        let style = html.style();
                        let _ = style.set_property("--px", &format!("{}px", f64::from(event.client_x()) - rect.left()));
                        let _ = style.set_property("--py", &format!("{}px", f64::from(event.client_y()) - rect.top()));
                    }
                }
                on:pointerleave=move |_| set_hovered.set(None)>
                {PROJECTS.iter().enumerate().map(|(index, project)| view! {
                    <WorkRow index project open set_open set_hovered hidden=move || { index >= VISIBLE_PROJECTS && !show_all.get() } />
                }).collect_view()}
                <li class="work-preview" aria-hidden="true" class:visible=move || hovered.get().is_some_and(|index| open.get() != Some(index))>
                    {move || hovered.get().map(|index| view! { <ProjectVisual project=&PROJECTS[index] /> })}
                </li>
            </ol>
            <button class="more-button" aria-expanded=move || show_all.get().to_string() on:click=move |_| {
                if show_all.get_untracked() {
                    set_open.update(|current| if current.is_some_and(|index| index >= VISIBLE_PROJECTS) { *current = None });
                }
                set_show_all.update(|value| *value = !*value);
            }>
                {move || if show_all.get() { "show fewer".to_string() } else { format!("show all {} projects", PROJECTS.len()) }}
                <span class="more-icon" aria-hidden="true"></span>
            </button>
        </section>
    }
}

#[component]
fn WorkRow(
    index: usize,
    project: &'static Project,
    open: ReadSignal<Option<usize>>,
    set_open: WriteSignal<Option<usize>>,
    set_hovered: WriteSignal<Option<usize>>,
    hidden: impl Fn() -> bool + Send + Sync + Copy + 'static,
) -> impl IntoView {
    let is_open = move || open.get() == Some(index);
    let panel_id = format!("work-panel-{index}");
    let row_id = format!("work-row-{index}");
    view! {
        <li class="work-item" class:open=is_open hidden=hidden on:pointerenter=move |_| set_hovered.set(Some(index))>
            <h3>
                <button id=row_id.clone() class="work-row" aria-expanded=move || is_open().to_string() aria-controls=panel_id.clone()
                    on:click=move |_| set_open.update(|current| *current = if *current == Some(index) { None } else { Some(index) })>
                    <span class="work-index">{format!("{:02}", index + 1)}</span>
                    <span class="work-name" title=project.name>{project.name}</span>
                    <span class="work-category">{project.category}</span>
                    <span class="work-status">{project.status}</span>
                    <span class="work-toggle" aria-hidden="true"></span>
                </button>
            </h3>
            <div id=panel_id class="work-panel" role="region" aria-labelledby=row_id inert=move || !is_open()>
                <div class="work-panel-inner">
                    <div class="work-copy">
                        <p class="work-summary">{project.summary}</p>
                        <dl>
                            <div><dt>"role"</dt><dd>{project.role}</dd></div>
                            <div><dt>"problem"</dt><dd>{project.problem}</dd></div>
                            <div><dt>"what i built"</dt><dd>{project.built}</dd></div>
                        </dl>
                        <ul class="chips" aria-label="Stack">{project.stack.split(", ").map(|item| view! { <li>{item}</li> }).collect_view()}</ul>
                        <div class="work-links">
                            {if project.links.is_empty() {
                                view! { <span class="private-note">"private repository"</span> }.into_any()
                            } else {
                                project.links.iter().map(|(label, href)| view! {
                                    <a class="arrow-link" href=*href target="_blank" rel="noopener noreferrer">{*label}<span class="sr-only">", opens in a new tab"</span><span aria-hidden="true">" \u{2197}"</span></a>
                                }).collect_view().into_any()
                            }}
                        </div>
                    </div>
                    <div class="work-visual"><ProjectVisual project /></div>
                </div>
            </div>
        </li>
    }
}

#[component]
fn ProjectVisual(project: &'static Project) -> impl IntoView {
    match project.image {
        Some(src) => view! { <img src=src alt="" loading="lazy" decoding="async" /> }.into_any(),
        None => view! { <div class="type-card" class:long={ project.name.len() > 14 }><span>{project.name}</span><small>{project.category}</small></div> }.into_any(),
    }
}

#[component]
fn Runs() -> impl IntoView {
    let reel = NodeRef::<Ul>::new();
    let scroll_reel = move |direction: f64| {
        if let Some(track) = reel.get() {
            let card = track
                .first_element_child()
                .map_or(320.0, |item| item.get_bounding_client_rect().width() + 20.0);
            let options = ScrollToOptions::new();
            options.set_left(direction * card);
            options.set_behavior(if reduced_motion() {
                ScrollBehavior::Auto
            } else {
                ScrollBehavior::Smooth
            });
            track.scroll_by_with_scroll_to_options(&options);
        }
    };
    view! {
        <section id="runs" class="section runs" aria-labelledby="runs-title">
            <header class="section-head">
                <h2 id="runs-title" class="section-title">"Outside the screen"</h2>
                <p class="section-lead">"Trail and road ultras. Flyovers play when they scroll into view. Hover a photo or video for the story, and download the route as GPX."</p>
                <div class="reel-controls">
                    <button class="round-button" aria-label="Previous posts" on:click=move |_| scroll_reel(-1.0)><span aria-hidden="true">"\u{2190}"</span></button>
                    <button class="round-button" aria-label="Next posts" on:click=move |_| scroll_reel(1.0)><span aria-hidden="true">"\u{2192}"</span></button>
                </div>
            </header>
            <ul node_ref=reel class="reel" tabindex="0" aria-label="Run posts, scroll horizontally">
                {RUNS.iter().enumerate().map(|(index, run)| view! { <li><RunCard index run /></li> }).collect_view()}
            </ul>
        </section>
    }
}

#[component]
fn RunCard(index: usize, run: &'static RunPost) -> impl IntoView {
    let (slide, set_slide) = signal(0usize);
    let (details, set_details) = signal(false);
    let count = run.slides.len();
    let title_id = format!("run-title-{index}");
    let labelled_by = title_id.clone();
    view! {
        <article class="post" aria-labelledby=labelled_by>
            <header class="post-head">
                <img src="assets/profile.jpg" width="32" height="32" alt="" loading="lazy" decoding="async" />
                <div><strong>"madrid"</strong><span>{run.place}</span></div>
                {(!run.date.is_empty()).then(|| view! { <time>{run.date}</time> })}
            </header>
            <div class="post-media" class:show-details=move || details.get()>
                <div class="slides" data-active=move || slide.get().to_string()>
                    {run.slides.iter().enumerate().map(|(position, item)| view! { <RunSlide item position active=slide note=run.note /> }).collect_view()}
                </div>
                {(count > 1).then(|| view! {
                    <div class="slide-bars" aria-hidden="true">
                        {(0..count).map(|position| view! { <span class:done=move || { slide.get() >= position }></span> }).collect_view()}
                    </div>
                    <button class="slide-nav prev" aria-label="Previous slide" disabled=move || slide.get() == 0 on:click=move |_| set_slide.update(|value| *value = value.saturating_sub(1))><span aria-hidden="true">"\u{2039}"</span></button>
                    <button class="slide-nav next" aria-label="Next slide" disabled=move || { slide.get() + 1 >= count } on:click=move |_| set_slide.update(|value| *value = (*value + 1).min(count - 1))><span aria-hidden="true">"\u{203A}"</span></button>
                })}
                {run.gpx.map(|(href, size)| {
                    let file = href.rsplit('/').next().unwrap_or("route.gpx");
                    view! { <a class="gpx-chip" href=href download=file type="application/gpx+xml" aria-label=format!("Download GPX route, {size}")><span aria-hidden="true">"\u{2193} GPX"</span><small aria-hidden="true">{size}</small></a> }
                })}
                <button class="details-toggle" aria-pressed=move || details.get().to_string() aria-label="Show description" on:click=move |_| set_details.update(|value| *value = !*value)><span aria-hidden="true">"i"</span></button>
                <p class="sr-only" aria-live="polite">{move || format!("Slide {} of {count}", slide.get() + 1)}</p>
            </div>
            <div class="post-body">
                <h3 id=title_id>{run.title}</h3>
                <ul class="post-stats">
                    {run.stats.iter().map(|(value, label)| view! { <li><strong>{*value}</strong><span>{*label}</span></li> }).collect_view()}
                </ul>
            </div>
        </article>
    }
}

#[component]
fn RunSlide(
    item: &'static Slide,
    position: usize,
    active: ReadSignal<usize>,
    note: &'static str,
) -> impl IntoView {
    let inactive = move || active.get() != position;
    match item {
        Slide::Video { src, poster, label, description } => {
            view! { <figure class="slide" aria-hidden=move || inactive().then_some("true")><ReelVideo src=*src poster=*poster label=*label /><figcaption class="slide-desc"><strong>{note}</strong>{*description}</figcaption></figure> }.into_any()
        }
        Slide::Image { src, alt, description } => {
            view! { <figure class="slide" aria-hidden=move || inactive().then_some("true")><img class="slide-backdrop" src=*src alt="" loading="lazy" decoding="async" /><img class="contain" src=*src alt=*alt loading="lazy" decoding="async" /><figcaption class="slide-desc"><strong>{note}</strong>{*description}</figcaption></figure> }.into_any()
        }
    }
}

#[component]
fn ReelVideo(src: &'static str, poster: &'static str, label: &'static str) -> impl IntoView {
    let video = NodeRef::<Video>::new();
    let (muted, set_muted) = signal(true);
    let (progress, set_progress) = signal(0.0_f64);
    let manual = reduced_motion();
    Effect::new(move |_| {
        if !manual && let Some(element) = video.get() {
            observe_video(&element);
        }
    });
    view! {
        <video node_ref=video src=src poster=poster preload="none" muted=true loop=true playsinline=true controls=manual aria-label=label
            prop:muted=move || muted.get()
            on:timeupdate=move |_| {
                if let Some(element) = video.get() {
                    let duration = element.duration();
                    if duration.is_finite() && duration > 0.0 { set_progress.set(element.current_time() / duration); }
                }
            }></video>
        {(!manual).then(|| view! {
            <span class="video-progress" aria-hidden="true"><span style:transform=move || format!("scaleX({:.4})", progress.get())></span></span>
            <button class="sound-toggle" aria-pressed=move || (!muted.get()).to_string() aria-label="Sound" on:click=move |_| {
                set_muted.update(|value| *value = !*value);
                if let Some(element) = video.get() { play_media(&element); }
            }><span aria-hidden="true">{move || if muted.get() { "sound off" } else { "sound on" }}</span></button>
        })}
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section id="about" class="section about" aria-labelledby="about-title">
            <header class="section-head"><h2 id="about-title" class="section-title">"About"</h2></header>
            <div class="about-grid">
                <p class="about-statement">
                    "I study "<mark>"Data Science at NUS"</mark>" and build practical tools around "<mark>"maps, language"</mark>" and everyday problems. Away from a screen, I run "<mark>"trail ultras"</mark>". Open to internships, collaborations and useful problems."
                </p>
                <figure class="about-figure">
                    <img src="assets/media/shoe.webp" width="640" height="642" alt="Madrid holding a trail shoe, wearing a running vest and cap" loading="lazy" decoding="async" />
                    <figcaption class="sticker">"trail mode"</figcaption>
                </figure>
            </div>
            <div class="about-columns">
                <div>
                    <h3>"Education"</h3>
                    <p>"NUS, B.Sc. Data Science, year 3. GPA 4.39/5.00, best semester 4.80."</p>
                </div>
                <div>
                    <h3>"Experience"</h3>
                    <ul>
                        <li><strong>"NUS content creator."</strong>" Produced blended learning videos for TCX1002 with Prof. Jiang Kan, from scripting through editing."</li>
                        <li><strong>"BearlyFunGym."</strong>" Coached children through milestone based lessons and kept parents informed."</li>
                        <li><strong>"AUA Tsinghua."</strong>" Selected participant in the Asia Universities Alliance program at Tsinghua University."</li>
                    </ul>
                </div>
                <div>
                    <h3>"Races"</h3>
                    <ul class="race-list">
                        <li class="upcoming"><span>"BTS170, East Java, Nov 2026"</span><strong>"170 km"</strong></li>
                        <li><span>"Singapore loop for SHINE"</span><strong>"202.54 km"</strong></li>
                        <li><span>"BUS Backyard Ultra, third"</span><strong>"83.52 km"</strong></li>
                        <li><span>"Rinjani, first race"</span><strong>"61.27 km"</strong></li>
                        <li><span>"CULTRA, highest placing Singaporean"</span><strong>"60 km"</strong></li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocumentLinks() -> impl IntoView {
    view! {
        <ul class="doc-links">
            {DOCUMENTS.iter().map(|document| {
                let file = document.href.rsplit('/').next().unwrap_or("document.pdf");
                view! {
                    <li>
                        <a class="doc-card" href=document.href download=file aria-label=format!("Download {}, {}", document.title, document.meta)>
                            <span class="doc-sheet" aria-hidden="true"><b>"PDF"</b></span>
                            <span class="doc-text" aria-hidden="true">
                                <strong>{document.title}</strong>
                                <span>{document.detail}</span>
                                <small>{document.meta}</small>
                            </span>
                            <span class="doc-arrow" aria-hidden="true">"\u{2193}"</span>
                        </a>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}

#[component]
fn Footer(open_egg: impl Fn() + Copy + 'static) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (topic, set_topic) = signal("Internship".to_string());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal(String::new());
    view! {
        <footer class="footer">
            <p class="footer-title" aria-hidden="true">"say hi"</p>
            <section id="contact" class="contact" aria-labelledby="contact-title">
                <div class="contact-intro">
                    <h2 id="contact-title">"Write to me"</h2>
                    <p class="footer-lead">"Internships, collaborations, a race to try, or a trip to India or Africa. Messages go to my NUS inbox."</p>
                    <a class="email-link" href=format!("mailto:{EMAIL}")>{EMAIL}</a>
                    <DocumentLinks />
                </div>
                <form class="contact-form" on:submit=move |event| {
                    event.prevent_default();
                    let body_text = message.get_untracked();
                    if body_text.trim().is_empty() {
                        set_status.set("Add a short message first.".to_string());
                        return;
                    }
                    let sender = name.get_untracked();
                    let subject = if sender.trim().is_empty() { format!("{} via madrid-lim", topic.get_untracked()) } else { format!("{} from {}", topic.get_untracked(), sender.trim()) };
                    let href = format!("mailto:{EMAIL}?subject={}&body={}", js_sys::encode_uri_component(&subject), js_sys::encode_uri_component(body_text.trim()));
                    if let Some(window) = web_sys::window() && window.location().set_href(&href).is_ok() {
                        set_status.set(format!("Opening your email app. If nothing opens, write to {EMAIL}."));
                    } else {
                        set_status.set(format!("Could not open an email app. Write to {EMAIL}."));
                    }
                }>
                    <p class="madlib">
                        "Hi Madrid, I'm "
                        <input name="name" aria-label="Your name" placeholder="your name" autocomplete="name" maxlength="80" size="10" prop:value=move || name.get() on:input=move |event| set_name.set(event_target_value(&event)) />
                        " and I'm writing about "
                        <select name="topic" aria-label="Topic" on:change=move |event| set_topic.set(event_target_value(&event))>
                            {[("Internship", "an internship"), ("Collaboration", "a collaboration"), ("Race or trail tip", "a race or trail"), ("Hello", "just saying hi")].into_iter().map(|(value, label)| view! { <option value=value selected=move || topic.get() == value>{label}</option> }).collect_view()}
                        </select>
                        "."
                    </p>
                    <textarea name="message" aria-label="Message" placeholder="Tell me what you have in mind..." rows="4" maxlength="2000" required=true prop:value=move || message.get() on:input=move |event| set_message.set(event_target_value(&event))></textarea>
                    <button type="submit" class="send-button">"send with your email app"<span aria-hidden="true">" \u{2192}"</span></button>
                    <p class="form-status" role="status">{move || status.get()}</p>
                </form>
            </section>
            <ul class="footer-links">
                {CONTACTS.into_iter().map(|(label, href)| view! {
                    <li><a href=href target="_blank" rel="noopener noreferrer"><SocialIcon name=label />{label}<span class="sr-only">", opens in a new tab"</span></a></li>
                }).collect_view()}
            </ul>
            <div class="footer-meta">
                <span>"Madrid Lim, 2026"</span>
                <button class="secret-button" on:click=move |_| open_egg()><span aria-hidden="true">"\u{2726} "</span>"psst, there's a secret"</button>
                <a href="#top">"back to top \u{2191}"</a>
            </div>
        </footer>
    }
}

#[component]
fn NextUp(close: impl Fn() + Copy + 'static) -> impl IntoView {
    let close_button = NodeRef::<Button>::new();
    Effect::new(move |_| {
        if let Some(button) = close_button.get() {
            let _ = button.focus();
        }
    });
    view! {
        <div class="overlay" role="presentation" on:mousedown=move |event| { if event.target() == event.current_target() { close(); } }>
            <section class="next-up" role="dialog" aria-modal="true" aria-labelledby="next-title" on:keydown=move |event: KeyboardEvent| {
                if event.key() == "Escape" { event.prevent_default(); close(); }
                if event.key() == "Tab" {
                    let Some(dialog) = event.current_target().and_then(|target| target.dyn_into::<web_sys::Element>().ok()) else { return };
                    let Ok(items) = dialog.query_selector_all("button, a[href]") else { return };
                    let first = items.item(0).and_then(|node| node.dyn_into::<HtmlElement>().ok());
                    let last = items.item(items.length().saturating_sub(1)).and_then(|node| node.dyn_into::<HtmlElement>().ok());
                    let active = web_sys::window().and_then(|window| window.document()).and_then(|document| document.active_element());
                    let (edge, target) = if event.shift_key() { (first, last) } else { (last, first) };
                    if edge.is_some_and(|edge| active.as_ref() == Some(edge.as_ref())) && let Some(target) = target {
                        event.prevent_default();
                        let _ = target.focus();
                    }
                }
            }>
                <div class="confetti" aria-hidden="true">{(0..18).map(|_| view! { <i></i> }).collect_view()}</div>
                <header>
                    <p class="next-kicker">"you found the secret page"</p>
                    <h2 id="next-title">"Next up"</h2>
                    <button node_ref=close_button class="round-button close" aria-label="Close" on:click=move |_| close()><span aria-hidden="true">"\u{00D7}"</span></button>
                </header>
                <ol class="tickets">
                    <li class="ticket bib">
                        <span class="ticket-label">"race bib"</span>
                        <strong>"BTS170"</strong>
                        <span>"Bromo Tengger Semeru, East Java"</span>
                        <span class="ticket-meta"><b>"170K"</b><b>"Nov 7 to 8, 2026"</b></span>
                    </li>
                    <li class="ticket launch">
                        <span class="ticket-label">"launch ticket"</span>
                        <strong>"Sheng"</strong>
                        <span>"Public launch for language learning from the videos you already watch."</span>
                        <span class="ticket-meta"><b>"desktop"</b><b>"end of 2026"</b></span>
                    </li>
                    <li class="ticket pass">
                        <span class="ticket-label">"boarding pass"</span>
                        <strong class="route">"SIN"<i aria-hidden="true">"\u{2708}"</i>"IND / AFR"</strong>
                        <span>"I want to spend time working and learning in India and Africa."</span>
                        <span class="ticket-meta"><b>"gate: your inbox"</b><b>"seat: open"</b></span>
                    </li>
                </ol>
                <a class="send-button" href="#contact" on:click=move |_| close()>"got an idea? say hi"<span aria-hidden="true">" \u{2192}"</span></a>
            </section>
        </div>
    }
}

fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches())
}

fn play_media(element: &HtmlMediaElement) {
    if let Ok(promise) = element.play() {
        IGNORE_REJECTION.with_borrow_mut(|handler| {
            let handler = handler.get_or_insert_with(|| Closure::new(|_: JsValue| {}));
            let _ = promise.catch(handler);
        });
    }
}

fn observe_video(element: &HtmlMediaElement) {
    VIDEO_OBSERVER.with_borrow_mut(|slot| {
        if slot.is_none() {
            let callback: ObserverCallback =
                Closure::new(|entries: js_sys::Array, _observer: IntersectionObserver| {
                    for entry in entries.iter() {
                        let Ok(entry) = entry.dyn_into::<IntersectionObserverEntry>() else {
                            continue;
                        };
                        let Ok(media) = entry.target().dyn_into::<HtmlMediaElement>() else {
                            continue;
                        };
                        if entry.intersection_ratio() >= 0.6 {
                            play_media(&media);
                        } else {
                            let _ = media.pause();
                        }
                    }
                });
            let options = IntersectionObserverInit::new();
            let thresholds = js_sys::Array::of2(&JsValue::from_f64(0.0), &JsValue::from_f64(0.6));
            options.set_threshold(&thresholds);
            if let Ok(observer) =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
            {
                *slot = Some((observer, callback));
            }
        }
        if let Some((observer, _)) = slot.as_ref() {
            observer.observe(element);
        }
    });
}

fn install_word_timer(set_word: WriteSignal<usize>) {
    if WORD_TIMER.with_borrow(Option::is_some) {
        return;
    }
    let callback = Closure::<dyn FnMut()>::new(move || {
        let hidden = web_sys::window()
            .and_then(|window| window.document())
            .is_some_and(|document| document.hidden());
        if !hidden {
            set_word.update(|index| *index = (*index + 1) % HERO_WORDS.len());
        }
    });
    if let Some(window) = web_sys::window()
        && window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                2600,
            )
            .is_ok()
    {
        WORD_TIMER.with_borrow_mut(|timer| *timer = Some(callback));
    }
}

fn install_scroll_spy(set_active: WriteSignal<&'static str>) {
    if SCROLL_LISTENER.with_borrow(Option::is_some) {
        return;
    }
    let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let line = window
            .inner_height()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0)
            * 0.4;
        let current = SECTIONS
            .iter()
            .rev()
            .find(|(id, _)| {
                document
                    .get_element_by_id(id)
                    .is_some_and(|element| element.get_bounding_client_rect().top() <= line)
            })
            .map_or("", |(id, _)| *id);
        set_active.set(current);
    });
    if let Some(window) = web_sys::window()
        && window
            .add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref())
            .is_ok()
    {
        SCROLL_LISTENER.with_borrow_mut(|listener| *listener = Some(callback));
    }
}

fn initial_theme() -> &'static str {
    if let Some(value) =
        browser_storage().and_then(|storage| storage.get_item("madrid-theme").ok().flatten())
    {
        return if value == "dark" { "dark" } else { "light" };
    }
    let dark = web_sys::window()
        .and_then(|window| {
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
        })
        .is_some_and(|query| query.matches());
    if dark { "dark" } else { "light" }
}

fn store_theme(theme: &str) {
    if let Some(storage) = browser_storage() {
        let _ = storage.set_item("madrid-theme", theme);
    }
}

fn browser_storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn install_easter_eggs(
    egg_open: ReadSignal<bool>,
    open_egg: impl Fn() + Copy + 'static,
    close_egg: impl Fn() + Copy + 'static,
) {
    if GLOBAL_KEY_LISTENER.with_borrow(Option::is_some) {
        return;
    }
    let mut progress = 0usize;
    let callback = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        if egg_open.get_untracked() {
            if event.key() == "Escape" {
                event.prevent_default();
                close_egg();
            }
            return;
        }
        let typing = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|element| {
                matches!(
                    element.tag_name().to_lowercase().as_str(),
                    "input" | "textarea" | "select"
                ) || element.has_attribute("contenteditable")
            });
        if typing {
            progress = 0;
            return;
        }
        let key = event.key();
        let key = if key.len() == 1 {
            key.to_lowercase()
        } else {
            key
        };
        progress = if key == KONAMI[progress] {
            progress + 1
        } else if key == KONAMI[0] {
            1
        } else {
            0
        };
        if progress == KONAMI.len() {
            progress = 0;
            open_egg();
        }
    });
    if let Some(window) = web_sys::window()
        && window
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
            .is_ok()
    {
        GLOBAL_KEY_LISTENER.with_borrow_mut(|listener| *listener = Some(callback));
    }
}
