use std::cell::RefCell;

use leptos::html::{Button, Input, Section, Video};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlElement, KeyboardEvent, PointerEvent, Storage};

use crate::content::{CONTACTS, PROJECTS};

type KeyListener = Closure<dyn FnMut(KeyboardEvent)>;
type ResizeListener = Closure<dyn FnMut(web_sys::Event)>;

thread_local! {
    static GLOBAL_KEY_LISTENER: RefCell<Option<KeyListener>> = const { RefCell::new(None) };
    static RESIZE_LISTENER: RefCell<Option<ResizeListener>> = const { RefCell::new(None) };
}

const TECHNOLOGIES: [(&str, &str); 10] = [
    ("R", "Rust"),
    ("Py", "Python"),
    ("J", "Java"),
    ("S", "Swift"),
    ("PG", "PostgreSQL"),
    ("P+", "PostGIS"),
    ("S", "Supabase"),
    ("M", "MapLibre"),
    ("G", "Git"),
    ("D", "DaVinci Resolve"),
];

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = signal(initial_theme());
    let (tab, set_tab) = signal("projects");
    let (selected, set_selected) = signal(1usize);
    let (evidence_open, set_evidence_open) = signal(false);
    let (terminal_open, set_terminal_open) = signal(false);
    let (critter_message, set_critter_message) = signal(false);
    let (dragging, set_dragging) = signal(false);
    let (drag_origin, set_drag_origin) = signal((0.0_f64, 0.0_f64));
    let (window_origin, set_window_origin) = signal((0.0_f64, 0.0_f64));
    let (window_offset, set_window_offset) = signal((0.0_f64, 0.0_f64));
    let document_window = NodeRef::<leptos::html::Article>::new();
    let evidence_trigger = NodeRef::<Button>::new();
    let terminal_trigger = NodeRef::<Button>::new();
    let evidence_close = NodeRef::<Button>::new();
    let evidence_dialog = NodeRef::<Section>::new();
    let terminal_input = NodeRef::<Input>::new();
    let (critter_timeout, set_critter_timeout) = signal(None::<i32>);

    install_global_keys(
        evidence_open,
        set_evidence_open,
        evidence_trigger,
        terminal_open,
        set_terminal_open,
        terminal_trigger,
    );
    install_resize_reset(set_window_offset, set_dragging);

    Effect::new(move |_| {
        let open = evidence_open.get();
        if let Some(window) = document_window.get() {
            if open {
                let _ = window.set_attribute("inert", "");
                let _ = window.set_attribute("aria-hidden", "true");
            } else {
                let _ = window.remove_attribute("inert");
                let _ = window.remove_attribute("aria-hidden");
            }
        }
        if open && let Some(button) = evidence_close.get() {
            let _ = button.focus();
        }
    });
    Effect::new(move |_| {
        if terminal_open.get()
            && let Some(input) = terminal_input.get()
        {
            let _ = input.focus();
        }
    });
    Effect::new(move |_| {
        let color = if theme.get() == "dark" {
            "#161616"
        } else {
            "#e8e8e6"
        };
        if let Some(document) = web_sys::window().and_then(|window| window.document())
            && let Some(meta) = document.get_element_by_id("theme-color")
        {
            let _ = meta.set_attribute("content", color);
        }
    });
    Effect::new(move |_| {
        let (x, y) = window_offset.get();
        if let Some(element) = document_window.get() {
            let style = element.style();
            let _ = style.set_property("--drag-x", &format!("{x}px"));
            let _ = style.set_property("--drag-y", &format!("{y}px"));
        }
    });

    let close_evidence = move || {
        set_evidence_open.set(false);
        if let Some(button) = evidence_trigger.get() {
            let _ = button.focus();
        }
    };
    let close_terminal = move || {
        set_terminal_open.set(false);
        if let Some(button) = terminal_trigger.get() {
            let _ = button.focus();
        }
    };
    let toggle_theme = move || {
        let next = if theme.get_untracked() == "light" {
            "dark"
        } else {
            "light"
        };
        set_theme.set(next);
        store_theme(next);
    };
    let reset_window = move || {
        set_window_offset.set((0.0, 0.0));
        set_dragging.set(false);
    };
    let show_critter_message = move || {
        set_critter_message.set(true);
        if let Some(window) = web_sys::window() {
            if let Some(timeout) = critter_timeout.get_untracked() {
                window.clear_timeout_with_handle(timeout);
            }
            let callback = Closure::once(move || {
                set_critter_message.set(false);
                set_critter_timeout.set(None);
            });
            if let Ok(timeout) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                2000,
            ) {
                set_critter_timeout.set(Some(timeout));
                callback.forget();
            }
        }
    };

    view! {
        <div class="app-shell" data-theme=move || theme.get()>
            <div class="garden" aria-hidden="true">
                <div class="tree tree-left"></div><div class="tree tree-right"></div>
                <div class="shrub shrub-one"></div><div class="shrub shrub-two"></div><div class="shrub shrub-three"></div>
                <div class="flower flower-one"></div><div class="flower flower-two"></div><div class="flower flower-three"></div>
                <div class="stone stone-one"></div><div class="stone stone-two"></div>
            </div>
            <button class="critter chicken" aria-label="Find the garden chicken" on:click=move |_| show_critter_message()><span aria-hidden="true"></span></button>
            <button class="critter butterfly" aria-label="Find the garden butterfly" on:click=move |_| show_critter_message()><span aria-hidden="true"></span></button>
            <Show when=move || critter_message.get()><span class="critter-bubble" role="status">"found me."</span></Show>

            <article node_ref=document_window class="document-window" class:is-dragging=move || dragging.get() aria-label="Madrid Lim portfolio">
                <header class="titlebar"
                    on:pointerdown=move |event: PointerEvent| {
                        if event.button() != 0 || web_sys::window().is_none_or(|window| window.inner_width().ok().and_then(|value| value.as_f64()).unwrap_or(0.0) <= 800.0) { return; }
                        let Some(target) = event.target().and_then(|target| target.dyn_into::<web_sys::Element>().ok()) else { return; };
                        if target.closest("button, nav, a").ok().flatten().is_some() { return; }
                        if let Some(current) = event.current_target().and_then(|target| target.dyn_into::<HtmlElement>().ok()) {
                            let _ = current.set_pointer_capture(event.pointer_id());
                        }
                        set_drag_origin.set((event.client_x() as f64, event.client_y() as f64));
                        set_window_origin.set(window_offset.get_untracked());
                        set_dragging.set(true);
                        event.prevent_default();
                    }
                    on:pointermove=move |event: PointerEvent| {
                        if !dragging.get_untracked() { return; }
                        let (start_x, start_y) = drag_origin.get_untracked();
                        let (origin_x, origin_y) = window_origin.get_untracked();
                        let proposed_x = origin_x + event.client_x() as f64 - start_x;
                        let proposed_y = origin_y + event.client_y() as f64 - start_y;
                        if let (Some(window), Some(element)) = (web_sys::window(), document_window.get()) {
                            let viewport_w = window.inner_width().ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
                            let viewport_h = window.inner_height().ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
                            let rect = element.get_bounding_client_rect();
                            let current = window_offset.get_untracked();
                            let base_left = rect.left() - current.0;
                            let base_top = rect.top() - current.1;
                            let x = proposed_x.clamp(-base_left, viewport_w - base_left - rect.width());
                            let y = proposed_y.clamp(-base_top, viewport_h - base_top - rect.height());
                            set_window_offset.set((x, y));
                        }
                    }
                    on:pointerup=move |event: PointerEvent| {
                        set_dragging.set(false);
                        if let Some(current) = event.current_target().and_then(|target| target.dyn_into::<HtmlElement>().ok()) { let _ = current.release_pointer_capture(event.pointer_id()); }
                    }
                    on:pointercancel=move |_| set_dragging.set(false)
                    on:dblclick=move |event| {
                        let interactive = event.target().and_then(|target| target.dyn_into::<web_sys::Element>().ok()).is_some_and(|target| target.closest("button, nav, a").ok().flatten().is_some());
                        if !interactive { reset_window(); }
                    }>
                    <div class="traffic-lights" aria-hidden="true"><i></i><i></i><i></i></div>
                    <span class="document-title">"madrid.txt"</span>
                    <div class="title-actions">
                        <nav class="segments" aria-label="Portfolio sections">
                            {[("projects", "projects"), ("about", "about"), ("outside", "outside")].into_iter().map(|(value, label)| view! {
                                <button class:active=move || tab.get() == value aria-pressed=move || (tab.get() == value).to_string() on:click=move |_| set_tab.set(value)>{label}</button>
                            }).collect_view()}
                        </nav>
                        <button class="icon-button" aria-label=move || if theme.get() == "light" { "use dark theme" } else { "use light theme" } on:click=move |_| toggle_theme()>{move || if theme.get() == "light" { "◐" } else { "◑" }}</button>
                    </div>
                </header>

                <div class="window-body">
                    <aside class="identity">
                        <div>
                            <div class="identity-header"><img src="assets/profile.jpg" width="64" height="64" alt="Madrid Lim"/><div><h1>"Madrid Lim"</h1><p class="role">"Data Science at NUS"</p></div></div>
                            <p class="intro">"i build practical tools around maps, language and everyday problems."</p>
                            <p class="availability">"open to internships, collaborations and useful problems."</p>
                        </div>
                        <nav class="contacts" aria-label="Contact links">
                            {CONTACTS.into_iter().map(|(label, href)| view! { <a href=href target="_blank" rel="noopener noreferrer">{label}</a> }).collect_view()}
                        </nav>
                    </aside>

                    <main class="workspace">
                        {move || match tab.get() {
                            "about" => view! { <About on_outside=move || set_tab.set("outside") /> }.into_any(),
                            "outside" => view! { <Outside open_evidence=move || set_evidence_open.set(true) evidence_open trigger=evidence_trigger /> }.into_any(),
                            _ => view! { <Projects selected set_selected /> }.into_any(),
                        }}
                    </main>
                </div>

                <TechTicker open_terminal=move || set_terminal_open.set(true) terminal_trigger />
            </article>

            <Show when=move || evidence_open.get()>
                <div class="overlay evidence-overlay" role="presentation" on:mousedown=move |event| { if event.target() == event.current_target() { close_evidence(); } }>
                    <section node_ref=evidence_dialog id="evidence-dialog" class="dialog evidence-dialog" role="dialog" aria-modal="true" aria-labelledby="evidence-title" on:keydown=move |event: KeyboardEvent| {
                        if event.key() == "Tab"
                            && let Some(dialog) = evidence_dialog.get()
                            && let Ok(focusable) = dialog.query_selector_all("button, video[controls]")
                        {
                            let length = focusable.length();
                            if length > 0 {
                                let active = web_sys::window().and_then(|window| window.document()).and_then(|document| document.active_element());
                                let first = focusable.item(0).and_then(|node| node.dyn_into::<HtmlElement>().ok());
                                let last = focusable.item(length - 1).and_then(|node| node.dyn_into::<HtmlElement>().ok());
                                let wrap = if event.shift_key() { active == first.clone().map(Into::into) } else { active == last.clone().map(Into::into) };
                                if wrap {
                                    event.prevent_default();
                                    let target = if event.shift_key() { last } else { first };
                                    if let Some(target) = target { let _ = target.focus(); }
                                }
                            }
                        }
                    }>
                        <header><h2 id="evidence-title">"outside the screen"</h2><button node_ref=evidence_close on:click=move |_| close_evidence()>"close"</button></header>
                        <div class="evidence-grid">
                            <figure><img src="assets/evidence/rinjani.jpg" loading="lazy" decoding="async" alt="COROS Rinjani race record showing distance, time and elevation gain"/><figcaption>"Rinjani, first race. 61.27 km, 19:47:29, 5,287 m gain."</figcaption></figure>
                            <figure><img src="assets/evidence/running.jpg" loading="lazy" decoding="async" alt="Madrid running outdoors at night"/><figcaption>"Running during the 202.54 km journey around Singapore."</figcaption></figure>
                            <figure><img src="assets/evidence/singapore-202.jpg" loading="lazy" decoding="async" alt="COROS record of the 202.54 kilometre route around Singapore"/><figcaption>"202.54 km around Singapore in 46:46:23, raising $850 for SHINE."</figcaption></figure>
                            <EvidenceVideo name="Singapore" caption="Singapore route flyover." />
                            <EvidenceVideo name="Johor" caption="Johor route flyover." />
                            <EvidenceVideo name="Rinjani" caption="Rinjani route flyover." />
                        </div>
                    </section>
                </div>
            </Show>

            <Show when=move || terminal_open.get()>
                <Terminal input_ref=terminal_input close=close_terminal set_tab theme set_theme />
            </Show>
        </div>
    }
}

#[component]
fn TechTicker(
    open_terminal: impl Fn() + Copy + 'static,
    terminal_trigger: NodeRef<Button>,
) -> impl IntoView {
    let track = || {
        (0..2).flat_map(|_| TECHNOLOGIES).map(|(glyph, name)| view! { <span class="tech-item"><i aria-hidden="true">{glyph}</i>{name}</span> }).collect_view()
    };
    view! { <footer class="statusbar">
        <ul class="sr-only" aria-label="Technology stack">{TECHNOLOGIES.into_iter().map(|(_, name)| view! { <li>{name}</li> }).collect_view()}</ul>
        <div class="ticker" aria-hidden="true"><div class="ticker-track">{track()}</div><div class="ticker-track">{track()}</div></div>
        <button class="terminal-button" node_ref=terminal_trigger aria-label="Open terminal" on:click=move |_| open_terminal()>"/ terminal"</button>
    </footer> }
}

#[component]
fn EvidenceVideo(name: &'static str, caption: &'static str) -> impl IntoView {
    let (playing, set_playing) = signal(false);
    let video_ref = NodeRef::<Video>::new();
    Effect::new(move |_| {
        if playing.get()
            && let Some(video) = video_ref.get()
        {
            let _ = video.focus();
            let _ = video.play();
        }
    });
    let slug = name.to_lowercase();
    let poster = format!("assets/evidence/video/{slug}-flyover-poster.jpg");
    let poster_card = poster.clone();
    let source = format!("assets/evidence/video/{slug}-flyover.mp4");
    view! { <figure class="video-evidence">
        <Show when=move || playing.get() fallback=move || view! { <button class="video-poster" aria-label=format!("Play {name} route flyover") on:click=move |_| set_playing.set(true)><img src=poster_card.clone() loading="lazy" decoding="async" alt=""/><span>"play video"</span></button> }>
            <video node_ref=video_ref controls muted autoplay playsinline preload="none" poster=poster.clone() src=source.clone() aria-label=format!("{name} route flyover")></video>
        </Show>
        <figcaption>{caption}</figcaption>
    </figure> }
}

#[component]
fn Projects(selected: ReadSignal<usize>, set_selected: WriteSignal<usize>) -> impl IntoView {
    let option_refs = (0..PROJECTS.len())
        .map(|_| NodeRef::<Button>::new())
        .collect::<Vec<_>>();
    view! {
        <section class="projects" aria-label="Projects">
            <div class="project-list"><p class="section-label" id="projects-label">"selected work"</p><div class="options" role="listbox" aria-labelledby="projects-label">
                {PROJECTS.iter().enumerate().map(|(index, project)| {
                    let option_ref = option_refs[index]; let focus_refs = option_refs.clone();
                    view! { <button node_ref=option_ref role="option" tabindex=move || if selected.get() == index { 0 } else { -1 } aria-selected=move || (selected.get() == index).to_string() class:active=move || selected.get() == index on:click=move |_| set_selected.set(index) on:keydown=move |event: KeyboardEvent| {
                        let next = match event.key().as_str() { "ArrowDown" => Some((index + 1) % PROJECTS.len()), "ArrowUp" => Some((index + PROJECTS.len() - 1) % PROJECTS.len()), "Home" => Some(0), "End" => Some(PROJECTS.len() - 1), _ => None };
                        if let Some(next) = next { event.prevent_default(); set_selected.set(next); if let Some(button) = focus_refs[next].get() { let _ = button.focus(); } }
                    }><strong>{project.name}</strong><small>{project.category}</small></button> }
                }).collect_view()}
            </div></div>
            <ProjectDetail selected />
            <span class="sr-only" aria-live="polite">{move || format!("Selected project: {}", PROJECTS[selected.get()].name)}</span>
        </section>
    }
}

#[component]
fn ProjectDetail(selected: ReadSignal<usize>) -> impl IntoView {
    view! { <article class="project-detail">{move || { let project = &PROJECTS[selected.get()]; view! { <div class="detail-content" data-project=project.name>
        <p class="detail-category">{project.category}</p><h2>{project.name}</h2><p class="summary">{project.summary}</p>
        <dl><div><dt>"problem"</dt><dd>{project.problem}</dd></div><div><dt>"what i built"</dt><dd>{project.built}</dd></div><div><dt>"why it matters"</dt><dd>{project.why}</dd></div></dl>
        <p class="stack"><span>"stack"</span>{project.stack}</p>
        {project.href.map_or_else(|| view! { <span class="status-label">{project.status}</span> }.into_any(), |href| view! { <a class="project-link" href=href target="_blank" rel="noopener noreferrer">{project.status}</a> }.into_any())}
    </div> }}} </article> }
}

#[component]
fn About(on_outside: impl Fn() + Copy + 'static) -> impl IntoView {
    view! { <section class="reading-pane" aria-labelledby="about-title"><p class="section-label">"about"</p><h2 id="about-title">"Experience and education"</h2>
        <p class="pane-lead">"NUS B.Sc. Data Science, year 3 · GPA 4.39/5.00 · best semester 4.80 · A+ in CS1010S."</p>
        <div class="reading-item"><h3>"NUS content creator"</h3><p>"Produced blended learning videos for TCX1002 with Prof. Jiang Kan, from scripting through editing."</p></div>
        <div class="reading-item"><h3>"BearlyFunGym"</h3><p>"Coached children through milestone based lessons and kept parents informed."</p></div>
        <div class="reading-item"><h3>"AUA Tsinghua"</h3><p>"Selected participant in the Asia Universities Alliance program at Tsinghua University."</p></div>
        <button class="text-button" on:click=move |_| on_outside()>"outside the screen"</button>
    </section> }
}

#[component]
fn Outside(
    open_evidence: impl Fn() + Copy + 'static,
    evidence_open: ReadSignal<bool>,
    trigger: NodeRef<Button>,
) -> impl IntoView {
    view! { <section class="reading-pane outside-pane" aria-labelledby="outside-title"><h2 id="outside-title">"Outside the screen"</h2><p class="pane-lead">"Trail running tests planning and follow through away from a computer."</p>
        <ul class="fact-list"><li><strong>"Singapore"</strong><span>"202.54 km · 46:46:23 · $850 for SHINE"</span></li><li><strong>"Rinjani, first race"</strong><span>"61.27 km · 19:47:29 · 5,287 m gain"</span></li><li><strong>"BUS"</strong><span>"80 km · 12 loops · third"</span></li><li><strong>"CULTRA"</strong><span>"60 km · 9h25 · highest placing Singaporean"</span></li></ul>
        <button class="text-button" node_ref=trigger aria-haspopup="dialog" aria-controls="evidence-dialog" aria-expanded=move || evidence_open.get().to_string() on:click=move |_| open_evidence()>"view evidence"</button>
    </section> }
}

#[component]
fn Terminal(
    input_ref: NodeRef<Input>,
    close: impl Fn() + Copy + 'static,
    set_tab: WriteSignal<&'static str>,
    theme: ReadSignal<&'static str>,
    set_theme: WriteSignal<&'static str>,
) -> impl IntoView {
    let (lines, set_lines) = signal(vec!["type help for commands.".to_string()]);
    let (newest, set_newest) = signal("type help for commands.".to_string());
    let (draft, set_draft) = signal(String::new());
    let (history, set_history) = signal(Vec::<String>::new());
    let (history_index, set_history_index) = signal(0usize);
    let close_button = NodeRef::<Button>::new();
    view! { <div class="overlay terminal-overlay" role="presentation" on:mousedown=move |event| { if event.target() == event.current_target() { close(); } }><section class="dialog terminal-dialog" role="dialog" aria-modal="true" aria-labelledby="terminal-title" on:keydown=move |event: KeyboardEvent| {
        if event.key() == "Escape" { event.prevent_default(); close(); }
        if event.key() == "Tab" { event.prevent_default(); let on_close = event.target().and_then(|target| target.dyn_into::<HtmlElement>().ok()).is_some_and(|element| element.tag_name().eq_ignore_ascii_case("button")); if on_close { if let Some(input) = input_ref.get() { let _ = input.focus(); } } else if let Some(button) = close_button.get() { let _ = button.focus(); } }
    }><header><h2 id="terminal-title">"madrid.txt / terminal"</h2><button node_ref=close_button on:click=move |_| close()>"close"</button></header>
        <div class="terminal-history">{move || lines.get().into_iter().map(|line| view! { <p>{line}</p> }).collect_view()}<span class="sr-only" aria-live="polite">{move || newest.get()}</span></div>
        <form on:submit=move |event| { event.prevent_default(); let command = draft.get_untracked().trim().to_lowercase(); set_draft.set(String::new()); if command.is_empty() { return; } let output = match command.as_str() {
            "help" => "help, projects, about, outside, contact, theme, clear, close".to_string(), "projects" => { set_tab.set("projects"); "opened projects".to_string() }, "about" => { set_tab.set("about"); "opened about".to_string() }, "outside" => { set_tab.set("outside"); "opened outside".to_string() }, "contact" => "GitHub, LinkedIn, Instagram".to_string(), "theme" => { let next = if theme.get_untracked() == "light" { "dark" } else { "light" }; set_theme.set(next); store_theme(next); format!("theme: {next}") }, "clear" => { set_lines.set(Vec::new()); set_newest.set("cleared".to_string()); return; }, "close" => { close(); return; }, _ => "unknown command. type help.".to_string(),
        }; set_newest.set(output.clone()); set_lines.update(|items| { items.push(format!("> {command}")); items.push(output); if items.len() > 200 { let drop = items.len() - 200; items.drain(0..drop); } }); set_history.update(|items| { if items.len() >= 100 { items.remove(0); } items.push(command.clone()); }); set_history_index.set(history.get_untracked().len()); }>
            <label for="terminal-command">">"</label><input id="terminal-command" node_ref=input_ref maxlength="80" autocomplete="off" spellcheck="false" prop:value=move || draft.get() on:input=move |event| set_draft.set(event_target_value(&event)) on:keydown=move |event: KeyboardEvent| {
                if event.key() == "ArrowUp" { event.prevent_default(); let items = history.get_untracked(); if !items.is_empty() { let index = history_index.get_untracked().saturating_sub(1).min(items.len() - 1); set_history_index.set(index); set_draft.set(items[index].clone()); } }
                if event.key() == "ArrowDown" { event.prevent_default(); let items = history.get_untracked(); let index = (history_index.get_untracked() + 1).min(items.len()); set_history_index.set(index); set_draft.set(items.get(index).cloned().unwrap_or_default()); }
            } /></form>
    </section></div> }
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
fn install_resize_reset(
    set_window_offset: WriteSignal<(f64, f64)>,
    set_dragging: WriteSignal<bool>,
) {
    if RESIZE_LISTENER.with_borrow(Option::is_some) {
        return;
    }
    let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
        set_window_offset.set((0.0, 0.0));
        set_dragging.set(false);
    });
    if let Some(window) = web_sys::window()
        && window
            .add_event_listener_with_callback("resize", callback.as_ref().unchecked_ref())
            .is_ok()
    {
        RESIZE_LISTENER.with_borrow_mut(|listener| *listener = Some(callback));
    }
}

fn install_global_keys(
    evidence_open: ReadSignal<bool>,
    set_evidence_open: WriteSignal<bool>,
    evidence_trigger: NodeRef<Button>,
    terminal_open: ReadSignal<bool>,
    set_terminal_open: WriteSignal<bool>,
    terminal_trigger: NodeRef<Button>,
) {
    if GLOBAL_KEY_LISTENER.with_borrow(Option::is_some) {
        return;
    }
    let callback = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        if event.key() == "Escape" {
            if terminal_open.get_untracked() {
                event.prevent_default();
                set_terminal_open.set(false);
                if let Some(button) = terminal_trigger.get() {
                    let _ = button.focus();
                }
            } else if evidence_open.get_untracked() {
                event.prevent_default();
                set_evidence_open.set(false);
                if let Some(button) = evidence_trigger.get() {
                    let _ = button.focus();
                }
            }
            return;
        }
        if event.key() != "/" || event.ctrl_key() || event.meta_key() || event.alt_key() {
            return;
        }
        let typing = event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|element| {
                matches!(
                    element.tag_name().to_lowercase().as_str(),
                    "input" | "textarea"
                ) || element.has_attribute("contenteditable")
            });
        if !typing && !evidence_open.get_untracked() && !terminal_open.get_untracked() {
            event.prevent_default();
            set_terminal_open.set(true);
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
