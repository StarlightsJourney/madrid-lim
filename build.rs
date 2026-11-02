use std::collections::HashSet;
use std::fmt::{Display, Write as _};
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::{Deserialize, Deserializer};

const UPLOADS: &str = "assets/uploads/";
const OPTIMIZED: &str = "assets/optimized/";
const UPLOAD_IMAGES: [&str; 6] = ["jpg", "jpeg", "png", "webp", "heic", "heif"];
const UPLOAD_VIDEOS: [&str; 3] = ["mp4", "mov", "m4v"];
const PLATFORMS: [&str; 3] = ["GitHub", "LinkedIn", "Instagram"];
const TICKET_STYLES: [&str; 3] = ["bib", "launch", "pass"];
const SHOWCASE_SHAPES: [&str; 3] = ["screen", "phone", "page"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Default)]
struct Text(String);

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        Ok(Self(value.trim().to_string()))
    }
}

struct List<T>(Vec<T>);

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for List<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(
            Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default(),
        ))
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Project {
    name: Text,
    category: Text,
    status: Text,
    summary: Text,
    role: Text,
    problem: Text,
    built: Text,
    stack: List<Text>,
    links: List<Link>,
    image: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Link {
    label: Text,
    url: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Run {
    title: Text,
    place: Text,
    date: Text,
    stats: List<Stat>,
    note: Text,
    slides: List<Slide>,
    gpx: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Stat {
    value: Text,
    label: Text,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Slide {
    Video(VideoSlide),
    Image(ImageSlide),
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct VideoSlide {
    src: Text,
    poster: Text,
    label: Text,
    description: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct ImageSlide {
    src: Text,
    alt: Text,
    description: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Profile {
    hero: Hero,
    technologies: List<Text>,
    work_intro: Text,
    runs_intro: Text,
    about: About,
    contact: Contact,
    documents: List<Document>,
    next_up: List<Ticket>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Hero {
    words: List<HeroWord>,
    summary: Text,
    label_left: Text,
    label_right: Text,
    showcase: List<Showcase>,
    bubble: List<Bubble>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Showcase {
    label: Text,
    image: Text,
    url: Text,
    shape: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct HeroWord {
    text: Text,
    outline: Option<f64>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Bubble {
    first: Text,
    second: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct About {
    statement: Text,
    education: Text,
    experience: List<Experience>,
    races: List<Race>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Experience {
    title: Text,
    text: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Race {
    name: Text,
    distance: Text,
    upcoming: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Contact {
    intro: Text,
    email: Text,
    topics: List<Topic>,
    socials: List<Social>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Topic {
    subject: Text,
    label: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Social {
    platform: Text,
    url: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Document {
    title: Text,
    detail: Text,
    file: Text,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Ticket {
    style: Text,
    label: Text,
    title: Text,
    text: Text,
    meta: List<Text>,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Image,
    Video,
    Gpx,
    Pdf,
}

struct Checker {
    root: PathBuf,
    errors: Vec<String>,
    outputs: HashSet<String>,
}

impl Checker {
    fn error(&mut self, at: &str, message: impl Display) {
        self.errors.push(format!("{at}: {message}"));
    }

    fn text(&mut self, at: &str, value: &Text, required: bool) -> String {
        let text = value.0.as_str();
        if required && text.is_empty() {
            self.error(at, "is empty");
        }
        if text.contains(['\u{2012}', '\u{2013}', '\u{2014}', '\u{2015}']) {
            self.error(
                at,
                "uses a long dash. Use a comma, full stop or the word \"to\" instead",
            );
        }
        if looks_private(text) {
            self.error(
                at,
                "looks like a phone or student number. Remove it before publishing",
            );
        }
        lit(text)
    }

    fn link(&mut self, at: &str, value: &Text) -> String {
        if !value.0.starts_with("https://") || value.0.contains(char::is_whitespace) {
            self.error(
                at,
                format!("must be a full https:// link, got {:?}", value.0),
            );
        }
        lit(&value.0)
    }

    fn project_link(&mut self, at: &str, value: &Text) -> String {
        if value.0.trim_start_matches('/').starts_with("assets/") {
            lit(&self.file(at, value, Kind::Pdf).unwrap_or_default())
        } else {
            self.link(at, value)
        }
    }

    fn file(&mut self, at: &str, value: &Text, kind: Kind) -> Option<String> {
        let path = value.0.trim_start_matches('/');
        if path.is_empty() {
            self.error(at, "no file chosen");
            return None;
        }
        if path.split('/').any(|part| part == "..") || !path.starts_with("assets/") {
            self.error(at, format!("{path} must be a file inside assets/"));
            return None;
        }
        let source = self.root.join(path);
        println!("cargo::rerun-if-changed={}", source.display());
        if !source.is_file() {
            self.error(
                at,
                format!("{path} was not found. Upload it again or pick another file"),
            );
            return None;
        }
        let extension = path
            .rsplit_once('.')
            .map(|(_, ext)| ext.to_ascii_lowercase())
            .unwrap_or_default();
        let uploaded = path.strip_prefix(UPLOADS);
        let (allowed, converted): (&[&str], &str) = match (kind, uploaded) {
            (Kind::Image, Some(_)) => (&UPLOAD_IMAGES, "webp"),
            (Kind::Video, Some(_)) => (&UPLOAD_VIDEOS, "mp4"),
            (Kind::Image, None) => (&["webp", "jpg", "jpeg", "png"], ""),
            (Kind::Video, None) => (&["mp4"], ""),
            (Kind::Gpx, _) => (&["gpx"], ""),
            (Kind::Pdf, _) => (&["pdf"], ""),
        };
        if !allowed.contains(&extension.as_str()) {
            self.error(
                at,
                format!(
                    "{path} has the wrong file type. Use one of: {}",
                    allowed.join(", ")
                ),
            );
            return None;
        }
        let Some(rest) = uploaded.filter(|_| !converted.is_empty()) else {
            return Some(path.to_string());
        };
        let stem = rest.rsplit_once('.').map_or(rest, |(stem, _)| stem);
        let public = format!("{OPTIMIZED}{stem}.{converted}");
        if !self.outputs.insert(public.clone()) {
            self.error(
                at,
                format!(
                    "another upload is also named {stem}. Rename one of the files so the names differ"
                ),
            );
        }
        Some(public)
    }

    fn size(&self, path: &str) -> String {
        let bytes = fs::metadata(self.root.join(path)).map_or(0, |meta| meta.len());
        if bytes >= 1_000_000 {
            format!("{:.1} MB", bytes as f64 / 1_000_000.0)
        } else {
            let kb = ((bytes as f64 / 1024.0 / 10.0).round() as u64 * 10).max(10);
            format!("{kb} KB")
        }
    }
}

fn lit(text: &str) -> String {
    format!("{text:?}")
}

fn looks_private(text: &str) -> bool {
    let compact: Vec<char> = text
        .chars()
        .filter(|c| !matches!(c, ' ' | '-' | '+' | '(' | ')'))
        .collect();
    let mut run = 0;
    for c in &compact {
        run = if c.is_ascii_digit() { run + 1 } else { 0 };
        if run >= 8 {
            return true;
        }
    }
    compact.windows(9).any(|window| {
        window[0].eq_ignore_ascii_case(&'a')
            && window[1..8].iter().all(char::is_ascii_digit)
            && window[8].is_ascii_alphabetic()
    })
}

fn date(checker: &mut Checker, at: &str, value: &Text) -> String {
    if value.0.is_empty() {
        return lit("");
    }
    let parts: Vec<&str> = value.0.splitn(3, '-').collect();
    let parsed = match parts.as_slice() {
        [year, month, day] => year
            .parse::<u32>()
            .ok()
            .zip(month.parse::<usize>().ok())
            .zip(day.get(..2).unwrap_or(day).parse::<u32>().ok()),
        _ => None,
    };
    match parsed {
        Some(((year, month @ 1..=12), day @ 1..=31)) => {
            lit(&format!("{} {day}, {year}", MONTHS[month - 1]))
        }
        _ => {
            checker.error(at, format!("{:?} is not a date like 2026-05-02", value.0));
            lit("")
        }
    }
}

fn read<T: for<'de> Deserialize<'de>>(checker: &mut Checker, name: &str) -> Option<T> {
    let path = checker.root.join("content").join(name);
    println!("cargo::rerun-if-changed={}", path.display());
    let parsed = fs::read_to_string(&path)
        .map_err(|error| error.to_string())
        .and_then(|raw| serde_json::from_str(&raw).map_err(|error| error.to_string()));
    match parsed {
        Ok(value) => Some(value),
        Err(error) => {
            checker.error(&format!("content/{name}"), error);
            None
        }
    }
}

fn projects(checker: &mut Checker, projects: &[Project]) -> String {
    let mut names = HashSet::new();
    let mut out = String::from("pub const PROJECTS: &[Project] = &[\n");
    for (index, project) in projects.iter().enumerate() {
        let at = format!("Projects > {}", label(&project.name, index));
        if !project.name.0.is_empty() && !names.insert(project.name.0.clone()) {
            checker.error(&at, "another project has the same name");
        }
        let stack: Vec<String> = project
            .stack
            .0
            .iter()
            .map(|item| checker.text(&format!("{at} > stack"), item, true))
            .collect();
        let links: Vec<String> = project
            .links
            .0
            .iter()
            .map(|link| {
                let label = checker.text(&format!("{at} > link label"), &link.label, true);
                let url = checker.project_link(&format!("{at} > link url"), &link.url);
                format!("({label}, {url})")
            })
            .collect();
        let image = if project.image.0.is_empty() {
            "None".to_string()
        } else {
            checker
                .file(&format!("{at} > screenshot"), &project.image, Kind::Image)
                .map_or("None".to_string(), |path| format!("Some({})", lit(&path)))
        };
        let _ = writeln!(
            out,
            "    Project {{ name: {}, category: {}, role: {}, summary: {}, problem: {}, built: {}, stack: &[{}], links: &[{}], status: {}, image: {} }},",
            checker.text(&format!("{at} > name"), &project.name, true),
            checker.text(&format!("{at} > category"), &project.category, true),
            checker.text(&format!("{at} > role"), &project.role, true),
            checker.text(&format!("{at} > summary"), &project.summary, true),
            checker.text(&format!("{at} > problem"), &project.problem, true),
            checker.text(&format!("{at} > what i built"), &project.built, true),
            stack.join(", "),
            links.join(", "),
            checker.text(&format!("{at} > status"), &project.status, true),
            image,
        );
    }
    if projects.is_empty() {
        checker.error("Projects", "add at least one project");
    }
    out.push_str("];\n");
    out
}

fn runs(checker: &mut Checker, runs: &[Run]) -> String {
    let mut out = String::from("pub const RUNS: &[RunPost] = &[\n");
    let mut routes = HashSet::new();
    for (index, run) in runs.iter().enumerate() {
        let at = format!("Runs > {}", label(&run.title, index));
        let route = run.gpx.0.trim_start_matches('/');
        if !route.is_empty() && !routes.insert(route) {
            checker.error(
                &format!("{at} > GPX"),
                format!(
                    "{route} is already used by another run. Pick this run's own file or clear it"
                ),
            );
        }
        let stats: Vec<String> = run
            .stats
            .0
            .iter()
            .map(|stat| {
                format!(
                    "({}, {})",
                    checker.text(&format!("{at} > stat value"), &stat.value, true),
                    checker.text(&format!("{at} > stat label"), &stat.label, true)
                )
            })
            .collect();
        if run.slides.0.is_empty() {
            checker.error(&at, "add at least one photo or video");
        }
        let mut slides = Vec::new();
        for (position, slide) in run.slides.0.iter().enumerate() {
            let at = format!("{at} > slide {}", position + 1);
            match slide {
                Slide::Video(video) => {
                    let src = checker.file(&format!("{at} > video"), &video.src, Kind::Video);
                    let poster = if !video.poster.0.is_empty() {
                        checker.file(&format!("{at} > poster"), &video.poster, Kind::Image)
                    } else if let Some(stem) = src
                        .as_deref()
                        .and_then(|src| src.strip_prefix(OPTIMIZED))
                        .and_then(|src| src.strip_suffix(".mp4"))
                    {
                        Some(format!("{OPTIMIZED}{stem}-poster.webp"))
                    } else {
                        checker.error(&format!("{at} > poster"), "add a poster image");
                        None
                    };
                    slides.push(format!(
                        "Slide::Video {{ src: {}, poster: {}, label: {}, description: {} }}",
                        lit(src.as_deref().unwrap_or_default()),
                        lit(poster.as_deref().unwrap_or_default()),
                        checker.text(&format!("{at} > label"), &video.label, true),
                        checker.text(&format!("{at} > description"), &video.description, false),
                    ));
                }
                Slide::Image(image) => {
                    let src = checker.file(&format!("{at} > photo"), &image.src, Kind::Image);
                    slides.push(format!(
                        "Slide::Image {{ src: {}, alt: {}, description: {} }}",
                        lit(src.as_deref().unwrap_or_default()),
                        checker.text(&format!("{at} > alt text"), &image.alt, true),
                        checker.text(&format!("{at} > description"), &image.description, false),
                    ));
                }
            }
        }
        let gpx = if run.gpx.0.is_empty() {
            "None".to_string()
        } else {
            checker
                .file(&format!("{at} > GPX"), &run.gpx, Kind::Gpx)
                .map_or("None".to_string(), |path| {
                    format!("Some(({}, {}))", lit(&path), lit(&checker.size(&path)))
                })
        };
        let _ = writeln!(
            out,
            "    RunPost {{ title: {}, place: {}, date: {}, stats: &[{}], note: {}, slides: &[{}], gpx: {} }},",
            checker.text(&format!("{at} > title"), &run.title, true),
            checker.text(&format!("{at} > place"), &run.place, true),
            date(checker, &format!("{at} > date"), &run.date),
            stats.join(", "),
            checker.text(&format!("{at} > note"), &run.note, false),
            slides.join(", "),
            gpx,
        );
    }
    out.push_str("];\n");
    out
}

fn profile(checker: &mut Checker, profile: &Profile) -> String {
    let mut out = String::new();
    let hero = &profile.hero;
    if hero.words.0.is_empty() {
        checker.error("Profile > hero words", "add at least one word");
    }
    let words: Vec<String> = hero
        .words
        .0
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let at = format!("Profile > hero word {}", index + 1);
            let text = checker.text(&at, &word.text, true);
            let letters = word.text.0.chars().filter(|c| *c != ' ').count();
            let wanted = word.outline.unwrap_or(1.0);
            let position = word
                .text
                .0
                .char_indices()
                .filter(|(_, c)| *c != ' ')
                .nth((wanted as usize).saturating_sub(1))
                .filter(|_| wanted >= 1.0 && wanted.fract() == 0.0)
                .map(|(byte, _)| word.text.0[..byte].chars().count());
            let position = position.unwrap_or_else(|| {
                checker.error(
                    &format!("{at} > outlined letter"),
                    format!("must be a whole number from 1 to {letters}"),
                );
                0
            });
            format!("({text}, {position})")
        })
        .collect();
    if !(3..=8).contains(&hero.showcase.0.len()) {
        checker.error(
            "Profile > hero background projects",
            "needs between three and eight projects",
        );
    }
    let showcase: Vec<String> = hero
        .showcase
        .0
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let at = format!("Profile > hero background > {}", label(&item.label, index));
            if !SHOWCASE_SHAPES.contains(&item.shape.0.as_str()) {
                checker.error(
                    &format!("{at} > shape"),
                    format!("must be one of {}", SHOWCASE_SHAPES.join(", ")),
                );
            }
            let image = checker.file(&format!("{at} > image"), &item.image, Kind::Image);
            let href = checker.project_link(&format!("{at} > link"), &item.url);
            let local = item.url.0.trim_start_matches('/');
            let size = if local.starts_with("assets/") {
                checker.size(local)
            } else {
                String::new()
            };
            format!(
                "Showcase {{ label: {}, image: {}, href: {href}, shape: {}, size: {} }}",
                checker.text(&format!("{at} > label"), &item.label, true),
                lit(image.as_deref().unwrap_or_default()),
                lit(&item.shape.0),
                lit(&size)
            )
        })
        .collect();
    let mut bubble = vec![format!("({}, \"\")", lit("hi, i'm Madrid."))];
    for (index, line) in hero.bubble.0.iter().enumerate() {
        let at = format!("Profile > speech bubble {}", index + 1);
        bubble.push(format!(
            "({}, {})",
            checker.text(&format!("{at} > first line"), &line.first, true),
            checker.text(&format!("{at} > second line"), &line.second, false)
        ));
    }
    bubble.push(format!("({}, \"\")", lit("one more tap...")));
    let technologies: Vec<String> = profile
        .technologies
        .0
        .iter()
        .map(|item| checker.text("Profile > technologies", item, true))
        .collect();
    let about = &profile.about;
    checker.text("Profile > about statement", &about.statement, true);
    let parts: Vec<&str> = about.statement.0.split('*').collect();
    if parts.len().is_multiple_of(2) {
        checker.error(
            "Profile > about statement",
            "has an unmatched *. Wrap each highlight in a pair, like *trail ultras*",
        );
    }
    let segments: Vec<String> = parts
        .iter()
        .enumerate()
        .filter(|(_, part)| !part.is_empty())
        .map(|(index, part)| format!("({}, {})", lit(part), index % 2 == 1))
        .collect();
    let experience: Vec<String> = about
        .experience
        .0
        .iter()
        .map(|item| {
            format!(
                "({}, {})",
                checker.text("Profile > experience title", &item.title, true),
                checker.text("Profile > experience text", &item.text, true)
            )
        })
        .collect();
    let races: Vec<String> = about
        .races
        .0
        .iter()
        .map(|race| {
            format!(
                "Race {{ name: {}, distance: {}, upcoming: {} }}",
                checker.text("Profile > race name", &race.name, true),
                checker.text("Profile > race distance", &race.distance, true),
                race.upcoming.unwrap_or(false)
            )
        })
        .collect();
    let contact = &profile.contact;
    let email = &contact.email.0;
    let valid_email = email
        .split_once('@')
        .is_some_and(|(user, domain)| !user.is_empty() && domain.contains('.'))
        && !email.contains(char::is_whitespace);
    if !valid_email {
        checker.error(
            "Profile > email",
            format!("{email:?} is not an email address"),
        );
    }
    let topics: Vec<String> = contact
        .topics
        .0
        .iter()
        .map(|topic| {
            format!(
                "({}, {})",
                checker.text("Profile > contact topic subject", &topic.subject, true),
                checker.text("Profile > contact topic label", &topic.label, true)
            )
        })
        .collect();
    if topics.is_empty() {
        checker.error("Profile > contact topics", "add at least one topic");
    }
    let socials: Vec<String> = contact
        .socials
        .0
        .iter()
        .map(|social| {
            if !PLATFORMS.contains(&social.platform.0.as_str()) {
                checker.error(
                    "Profile > social links",
                    format!("platform must be one of {}", PLATFORMS.join(", ")),
                );
            }
            format!(
                "({}, {})",
                lit(&social.platform.0),
                checker.link("Profile > social link url", &social.url)
            )
        })
        .collect();
    let documents: Vec<String> = profile
        .documents
        .0
        .iter()
        .enumerate()
        .map(|(index, document)| {
            let at = format!("Profile > documents > {}", label(&document.title, index));
            let file = checker.file(&format!("{at} > file"), &document.file, Kind::Pdf);
            let meta = file
                .as_deref()
                .map(|path| format!("PDF \u{b7} {}", checker.size(path)))
                .unwrap_or_default();
            format!(
                "Document {{ title: {}, detail: {}, href: {}, meta: {} }}",
                checker.text(&format!("{at} > title"), &document.title, true),
                checker.text(&format!("{at} > detail"), &document.detail, true),
                lit(file.as_deref().unwrap_or_default()),
                lit(&meta)
            )
        })
        .collect();
    let tickets: Vec<String> = profile
        .next_up
        .0
        .iter()
        .enumerate()
        .map(|(index, ticket)| {
            let at = format!("Profile > next up > {}", label(&ticket.title, index));
            if !TICKET_STYLES.contains(&ticket.style.0.as_str()) {
                checker.error(
                    &format!("{at} > style"),
                    format!("must be one of {}", TICKET_STYLES.join(", ")),
                );
            }
            let meta: Vec<String> = ticket
                .meta
                .0
                .iter()
                .map(|item| checker.text(&format!("{at} > tags"), item, true))
                .collect();
            format!(
                "Ticket {{ style: {}, label: {}, title: {}, text: {}, meta: &[{}] }}",
                lit(&ticket.style.0),
                checker.text(&format!("{at} > label"), &ticket.label, true),
                checker.text(&format!("{at} > title"), &ticket.title, true),
                checker.text(&format!("{at} > text"), &ticket.text, true),
                meta.join(", ")
            )
        })
        .collect();

    let _ = writeln!(
        out,
        "pub const HERO_WORDS: &[(&str, usize)] = &[{}];",
        words.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const HERO_SUMMARY: &str = {};",
        checker.text("Profile > hero summary", &hero.summary, true)
    );
    let _ = writeln!(
        out,
        "pub const HERO_LABELS: (&str, &str) = ({}, {});",
        checker.text("Profile > hero left label", &hero.label_left, true),
        checker.text("Profile > hero right label", &hero.label_right, true)
    );
    let _ = writeln!(
        out,
        "pub const HERO_SHOWCASE: &[Showcase] = &[{}];",
        showcase.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const BUBBLE_LINES: &[(&str, &str)] = &[{}];",
        bubble.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const TECHNOLOGIES: &[&str] = &[{}];",
        technologies.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const WORK_INTRO: &str = {};",
        checker.text("Profile > work intro", &profile.work_intro, true)
    );
    let _ = writeln!(
        out,
        "pub const RUNS_INTRO: &str = {};",
        checker.text("Profile > runs intro", &profile.runs_intro, true)
    );
    let _ = writeln!(
        out,
        "pub const ABOUT_STATEMENT: &[(&str, bool)] = &[{}];",
        segments.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const EDUCATION: &str = {};",
        checker.text("Profile > education", &about.education, true)
    );
    let _ = writeln!(
        out,
        "pub const EXPERIENCE: &[(&str, &str)] = &[{}];",
        experience.join(", ")
    );
    let _ = writeln!(out, "pub const RACES: &[Race] = &[{}];", races.join(", "));
    let _ = writeln!(
        out,
        "pub const CONTACT_INTRO: &str = {};",
        checker.text("Profile > contact intro", &contact.intro, true)
    );
    let _ = writeln!(out, "pub const EMAIL: &str = {};", lit(email));
    let _ = writeln!(
        out,
        "pub const TOPICS: &[(&str, &str)] = &[{}];",
        topics.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const CONTACTS: &[(&str, &str)] = &[{}];",
        socials.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const DOCUMENTS: &[Document] = &[{}];",
        documents.join(", ")
    );
    let _ = writeln!(
        out,
        "pub const TICKETS: &[Ticket] = &[{}];",
        tickets.join(", ")
    );
    out
}

fn label(name: &Text, index: usize) -> String {
    if name.0.is_empty() {
        format!("item {}", index + 1)
    } else {
        name.0.clone()
    }
}

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", root.join("content").display());
    let mut checker = Checker {
        root,
        errors: Vec::new(),
        outputs: HashSet::new(),
    };
    let mut out = String::new();
    if let Some(items) = read::<Vec<Project>>(&mut checker, "projects.json") {
        out.push_str(&projects(&mut checker, &items));
    }
    if let Some(mut items) = read::<Vec<Run>>(&mut checker, "runs.json") {
        items.sort_by(|a, b| {
            (a.date.0.is_empty(), &a.date.0).cmp(&(b.date.0.is_empty(), &b.date.0))
        });
        out.push_str(&runs(&mut checker, &items));
    }
    if let Some(data) = read::<Profile>(&mut checker, "profile.json") {
        out.push_str(&profile(&mut checker, &data));
    }
    if !checker.errors.is_empty() {
        eprintln!("\nThe site content has problems. Fix these in the editor and save again:\n");
        for error in &checker.errors {
            eprintln!("  - {error}");
        }
        eprintln!();
        std::process::exit(1);
    }
    let target = Path::new(&env::var("OUT_DIR").unwrap_or_else(|_| ".".into())).join("content.rs");
    if let Err(error) = fs::write(&target, out) {
        eprintln!("could not write {}: {error}", target.display());
        std::process::exit(1);
    }
}
