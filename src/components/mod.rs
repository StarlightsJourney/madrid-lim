use std::cell::RefCell;

use leptos::html::{Button, Input};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlElement, KeyboardEvent, Storage};

use crate::content::{CONTACTS, PROJECTS};

type KeyListener = Closure<dyn FnMut(KeyboardEvent)>;

thread_local! {
    static GLOBAL_KEY_LISTENER: RefCell<Option<KeyListener>> = RefCell::new(None);
}

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = signal(initial_theme());
    let (tab, set_tab) = signal("projects");
    let (selected, set_selected) = signal(1usize);
    let (evidence_open, set_evidence_open) = signal(false);
    let (terminal_open, set_terminal_open) = signal(false);
    let evidence_trigger = NodeRef::<Button>::new();
    let terminal_trigger = NodeRef::<Button>::new();
    let evidence_close = NodeRef::<Button>::new();
    let terminal_input = NodeRef::<Input>::new();

    install_global_keys(
        evidence_open,
        set_evidence_open,
        evidence_trigger,
        terminal_open,
        set_terminal_open,
        terminal_trigger,
    );

    Effect::new(move |_| {
        if evidence_open.get()
            && let Some(button) = evidence_close.get()
        {
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
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(meta) = document.get_element_by_id("theme-color")
        {
            let _ = meta.set_attribute("content", color);
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

    view! {
        <div class="app-shell" data-theme=move || theme.get()>
            <article class="document-window" aria-label="Madrid Lim portfolio">
                <header class="titlebar">
                    <div class="traffic-lights" aria-hidden="true"><i></i><i></i><i></i></div>
                    <span class="document-title">"madrid.txt"</span>
                    <div class="title-actions">
                        <nav class="segments" aria-label="Portfolio sections">
                            {[("projects", "projects"), ("about", "about"), ("outside", "outside")].into_iter().map(|(value, label)| view! {
                                <button class:active=move || tab.get() == value aria-pressed=move || (tab.get() == value).to_string() on:click=move |_| set_tab.set(value)>{label}</button>
                            }).collect_view()}
                        </nav>
                        <button class="icon-button" aria-label=move || if theme.get() == "light" { "use dark theme" } else { "use light theme" } on:click=move |_| toggle_theme()>{move || if theme.get() == "light" { "◐" } else { "◑" }}</button>
                        <button class="slash-button" node_ref=terminal_trigger aria-label="Open terminal" on:click=move |_| set_terminal_open.set(true)>"/"</button>
                    </div>
                </header>

                <div class="window-body">
                    <aside class="identity">
                        <div>
                            <h1>"Madrid Lim"</h1>
                            <p class="role">"Data Science at NUS"</p>
                            <p class="intro">"i build practical tools around maps, language and everyday problems."</p>
                            <p class="education">"NUS year 3"<br/>"GPA 4.39/5.00 · best semester 4.80"</p>
                            <p class="availability">"open to internships, collaborations and useful problems."</p>
                        </div>
                        <nav class="contacts" aria-label="Contact links">
                            {CONTACTS.into_iter().map(|(label, href)| {
                                view! { <a href=href target="_blank" rel="noopener noreferrer">{label}</a> }
                            }).collect_view()}
                        </nav>
                    </aside>

                    <main class="workspace">
                        {move || match tab.get() {
                            "about" => view! { <About on_outside=move || set_tab.set("outside") /> }.into_any(),
                            "outside" => view! { <Outside open_evidence=move || set_evidence_open.set(true) trigger=evidence_trigger /> }.into_any(),
                            _ => view! { <Projects selected set_selected /> }.into_any(),
                        }}
                    </main>
                </div>

                <footer class="statusbar">
                    <span>"202.54 km around singapore · $850 for shine"</span>
                    <button on:click=move |_| set_terminal_open.set(true)>"press / for terminal"</button>
                </footer>
            </article>

            <Show when=move || evidence_open.get()>
                <div class="overlay evidence-overlay" role="presentation" on:mousedown=move |event| { if event.target() == event.current_target() { close_evidence(); } }>
                    <section class="dialog evidence-dialog" role="dialog" aria-modal="true" aria-labelledby="evidence-title" on:keydown=move |event: KeyboardEvent| {
                        if event.key() == "Escape" { event.prevent_default(); close_evidence(); }
                        if event.key() == "Tab" { event.prevent_default(); if let Some(button) = evidence_close.get() { let _ = button.focus(); } }
                    }>
                        <header><h2 id="evidence-title">"outside the screen"</h2><button node_ref=evidence_close on:click=move |_| close_evidence()>"close"</button></header>
                        <div class="evidence-grid">
                            <figure><img src="assets/evidence/rinjani.jpg" loading="lazy" decoding="async" alt="COROS Rinjani race record showing distance, time and elevation gain"/><figcaption>"Rinjani, first race. 61.27 km, 19:47:29, 5,287 m gain."</figcaption></figure>
                            <figure><img src="assets/evidence/running.jpg" loading="lazy" decoding="async" alt="Madrid running outdoors at night"/><figcaption>"Running during the 202.54 km journey around Singapore."</figcaption></figure>
                            <figure><img src="assets/evidence/singapore-202.jpg" loading="lazy" decoding="async" alt="COROS record of the 202.54 kilometre route around Singapore"/><figcaption>"202.54 km around Singapore in 46:46:23, raising $850 for SHINE."</figcaption></figure>
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
fn Projects(selected: ReadSignal<usize>, set_selected: WriteSignal<usize>) -> impl IntoView {
    let option_refs = (0..PROJECTS.len())
        .map(|_| NodeRef::<Button>::new())
        .collect::<Vec<_>>();
    view! {
        <section class="projects" aria-label="Projects">
            <div class="project-list">
                <p class="section-label" id="projects-label">"selected work"</p>
                <div class="options" role="listbox" aria-labelledby="projects-label">
                {PROJECTS.iter().enumerate().map(|(index, project)| {
                    let option_ref = option_refs[index];
                    let focus_refs = option_refs.clone();
                    view! {
                    <button node_ref=option_ref role="option" tabindex=move || if selected.get() == index { 0 } else { -1 } aria-selected=move || (selected.get() == index).to_string() class:active=move || selected.get() == index on:click=move |_| set_selected.set(index) on:keydown=move |event: KeyboardEvent| {
                        let next = match event.key().as_str() {
                            "ArrowDown" => Some((index + 1) % PROJECTS.len()),
                            "ArrowUp" => Some((index + PROJECTS.len() - 1) % PROJECTS.len()),
                            "Home" => Some(0),
                            "End" => Some(PROJECTS.len() - 1),
                            _ => None,
                        };
                        if let Some(next) = next {
                            event.prevent_default();
                            set_selected.set(next);
                            if let Some(button) = focus_refs[next].get() { let _ = button.focus(); }
                        }
                    }>
                        <strong>{project.name}</strong><small>{project.category}</small>
                    </button>
                }}).collect_view()}
            </div>
            </div>
            <ProjectDetail selected />
            <span class="sr-only" aria-live="polite">{move || format!("Selected project: {}", PROJECTS[selected.get()].name)}</span>
        </section>
    }
}

#[component]
fn ProjectDetail(selected: ReadSignal<usize>) -> impl IntoView {
    view! { <article class="project-detail">{move || { let project = &PROJECTS[selected.get()]; view! {
        <p class="detail-category">{project.category}</p><h2>{project.name}</h2><p class="summary">{project.summary}</p>
        <dl><div><dt>"problem"</dt><dd>{project.problem}</dd></div><div><dt>"what i built"</dt><dd>{project.built}</dd></div><div><dt>"why it matters"</dt><dd>{project.why}</dd></div></dl>
        <p class="stack"><span>"stack"</span>{project.stack}</p>
        {project.href.map_or_else(|| view! { <span class="status-label">{project.status}</span> }.into_any(), |href| view! { <a class="project-link" href=href target="_blank" rel="noopener noreferrer">{project.status}</a> }.into_any())}
    }}} </article> }
}

#[component]
fn About(on_outside: impl Fn() + Copy + 'static) -> impl IntoView {
    view! { <section class="reading-pane" aria-labelledby="about-title"><p class="section-label">"about"</p><h2 id="about-title">"Experience and education"</h2>
        <p class="pane-lead">"NUS B.Sc. Data Science, year 3. GPA 4.39/5.00, best semester 4.80, A+ in CS1010S."</p>
        <div class="reading-item"><h3>"NUS content creator"</h3><p>"Produced blended learning videos for TCX1002 with Prof. Jiang Kan, from scripting through editing."</p></div>
        <div class="reading-item"><h3>"BearlyFunGym"</h3><p>"Coached children through milestone based lessons and kept parents informed."</p></div>
        <div class="reading-item"><h3>"AUA Tsinghua"</h3><p>"Selected participant in the Asia Universities Alliance program at Tsinghua University."</p></div>
        <div class="reading-item"><h3>"tools"</h3><p>"Python, Java, Rust, JavaScript, TypeScript, React Native, Expo, Supabase, Postgres, PostGIS, MapLibre, OneMap, Git, DaVinci Resolve, CapCut."</p></div>
        <button class="text-button" on:click=move |_| on_outside()>"outside the screen"</button>
    </section> }
}

#[component]
fn Outside(open_evidence: impl Fn() + Copy + 'static, trigger: NodeRef<Button>) -> impl IntoView {
    view! { <section class="reading-pane outside-pane" aria-labelledby="outside-title"><p class="section-label">"supporting proof"</p><h2 id="outside-title">"Outside the screen"</h2><p class="pane-lead">"Trail running tests patience, planning and follow through away from a computer."</p>
        <ul class="fact-list"><li><strong>"Singapore"</strong><span>"202.54 km · 46:46:23 · $850 for SHINE"</span></li><li><strong>"Rinjani, first race"</strong><span>"61.27 km · 19:47:29 · 5,287 m gain"</span></li><li><strong>"BUS"</strong><span>"80 km · 12 loops · third"</span></li><li><strong>"CULTRA"</strong><span>"60 km · 9h25 · highest placing Singaporean"</span></li></ul>
        <button class="text-button" node_ref=trigger aria-haspopup="dialog" on:click=move |_| open_evidence()>"open evidence"</button>
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
        if event.key() == "Tab" {
            event.prevent_default();
            let on_close = event.target()
                .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                .is_some_and(|element| element.tag_name().eq_ignore_ascii_case("button"));
            if on_close {
                if let Some(input) = input_ref.get() { let _ = input.focus(); }
            } else if let Some(button) = close_button.get() { let _ = button.focus(); }
        }
    }><header><h2 id="terminal-title">"madrid.txt / terminal"</h2><button node_ref=close_button on:click=move |_| close()>"close"</button></header>
        <div class="terminal-history">{move || lines.get().into_iter().map(|line| view! { <p>{line}</p> }).collect_view()}<span class="sr-only" aria-live="polite">{move || newest.get()}</span></div>
        <form on:submit=move |event| { event.prevent_default(); let command = draft.get_untracked().trim().to_lowercase(); set_draft.set(String::new()); if command.is_empty() { return; } let output = match command.as_str() {
            "help" => "help, projects, about, outside, contact, theme, clear, close".to_string(),
            "projects" => { set_tab.set("projects"); "opened projects".to_string() },
            "about" => { set_tab.set("about"); "opened about".to_string() },
            "outside" => { set_tab.set("outside"); "opened outside".to_string() },
            "contact" => "GitHub, LinkedIn, Instagram".to_string(),
            "theme" => { let next = if theme.get_untracked() == "light" { "dark" } else { "light" }; set_theme.set(next); store_theme(next); format!("theme: {next}") },
            "clear" => { set_lines.set(Vec::new()); set_newest.set("cleared".to_string()); return; },
            "close" => { close(); return; },
            _ => "unknown command. type help.".to_string(),
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
        if !typing && !evidence_open.get_untracked() {
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
