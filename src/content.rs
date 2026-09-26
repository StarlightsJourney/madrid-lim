#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

pub struct Project {
    pub name: &'static str,
    pub category: &'static str,
    pub role: &'static str,
    pub summary: &'static str,
    pub problem: &'static str,
    pub built: &'static str,
    pub stack: &'static str,
    pub links: &'static [(&'static str, &'static str)],
    pub status: &'static str,
    pub image: Option<&'static str>,
}

pub const PROJECTS: [Project; 8] = [
    Project {
        name: "Linguini",
        category: "photo-led language app",
        role: "Team of four. I owned the landing page, marketing, user analytics and the evaluation dataset.",
        summary: "Learn the words for what is in your own photos.",
        problem: "Fixed syllabus apps teach words a learner may never use.",
        built: "The landing page, blog and SEO pages, the launch film and campaign, the analytics write-up, and the image set used to evaluate scene analysis.",
        stack: "typescript, python, supabase, openrouter",
        links: &[
            ("live app", "https://linguini-navy.vercel.app/"),
            ("landing page", "https://linguini-landing.vercel.app/"),
            ("GitHub", "https://github.com/CS3216-A3-G7/linguini"),
        ],
        status: "shipped, team",
        image: Some("assets/work/linguini-landing.webp"),
    },
    Project {
        name: "HillGPX",
        category: "maps and elevation",
        role: "Solo. Data pipeline, design and frontend.",
        summary: "A map of hills, staircases and tall blocks for finding elevation gain to train on.",
        problem: "Route platforms make it hard to find climbs by terrain.",
        built: "Search across 17,747 venues, runnable routes, and a GPX profiler that runs in the browser and never uploads the track.",
        stack: "react, typescript, maplibre, python",
        links: &[("GitHub", "https://github.com/StarlightsJourney/HillGPX")],
        status: "public",
        image: Some("assets/work/hillgpx.webp"),
    },
    Project {
        name: "Ka-teng",
        category: "family lineage",
        role: "Solo. Design and frontend.",
        summary: "An open source family tree that stays readable as families grow.",
        problem: "Family history is hard to navigate in spreadsheets and chat threads.",
        built: "A navigable 2D tree with search and flows to add or connect parents, spouses and children.",
        stack: "react, typescript, vite",
        links: &[("GitHub", "https://github.com/StarlightsJourney/Ka-teng")],
        status: "public",
        image: Some("assets/work/kateng.webp"),
    },
    Project {
        name: "Sheng",
        category: "desktop language capture",
        role: "Solo. Architecture and implementation.",
        summary: "Learn from the videos you already watch. Public launch planned for the end of 2026.",
        problem: "Looking up a line and turning it into something reviewable breaks the flow.",
        built: "Capture, transcription, dictionary lookup and spaced review in one offline-first desktop app.",
        stack: "rust, tauri, svelte, sqlite",
        links: &[],
        status: "launching 2026",
        image: Some("assets/work/sheng.webp"),
    },
    Project {
        name: "CivicTwin",
        category: "policy simulation",
        role: "Contributor. I redesigned the frontend transport workflow.",
        summary: "A simulator that finds the residents a transport policy quietly harms.",
        problem: "Averages hide the few people a policy change hurts most.",
        built: "A redesign of the policy, simulation, impact, voices and consultation screens across 23 frontend files.",
        stack: "react, typescript, python",
        links: &[("GitHub", "https://github.com/Armaan-king/CivicTwin")],
        status: "contributor",
        image: Some("assets/work/civictwin.webp"),
    },
    Project {
        name: "NUS Timetable Optimizer",
        category: "constraint scheduling",
        role: "Solo. Scoring model and bot.",
        summary: "Rank timetables by the tradeoffs students care about.",
        problem: "A clash free timetable can still have bad gaps, early lessons and long walks.",
        built: "Constraint filtering and weighted scoring over module and venue data.",
        stack: "python, nusmods, telegram",
        links: &[],
        status: "private",
        image: None,
    },
    Project {
        name: "HejAmigo",
        category: "local marketplace",
        role: "Solo. Product and mobile app.",
        summary: "A Denmark first marketplace for local errands.",
        problem: "Small local tasks need a simple way to match requesters and helpers.",
        built: "Map based discovery, task posting, offers, accepted task chat, profiles and reviews.",
        stack: "expo, react native, supabase, maps",
        links: &[],
        status: "in development",
        image: None,
    },
    Project {
        name: "NoSleepMenuBar",
        category: "macOS utility",
        role: "Solo.",
        summary: "One click keeps a Mac awake.",
        problem: "A broken lid sensor made macOS sleep unreliable.",
        built: "A native menu bar app with timed presets that restores normal sleep when it stops.",
        stack: "swift, macOS",
        links: &[(
            "GitHub",
            "https://github.com/StarlightsJourney/NoSleepMenuBar",
        )],
        status: "public",
        image: None,
    },
];

pub enum Slide {
    Video {
        src: &'static str,
        poster: &'static str,
        label: &'static str,
        description: &'static str,
    },
    Image {
        src: &'static str,
        alt: &'static str,
        description: &'static str,
    },
}

pub struct RunPost {
    pub title: &'static str,
    pub place: &'static str,
    pub date: &'static str,
    pub stats: &'static [(&'static str, &'static str)],
    pub note: &'static str,
    pub slides: &'static [Slide],
    pub gpx: Option<(&'static str, &'static str)>,
}

pub const RUNS: [RunPost; 4] = [
    RunPost {
        title: "North Lombok Regency Trail Run",
        place: "Mount Rinjani, Lombok",
        date: "May 2, 2026",
        stats: &[
            ("61.27 km", "distance"),
            ("19:47:29", "time"),
            ("5,287 m", "gain"),
        ],
        note: "First race.",
        slides: &[
            Slide::Video {
                src: "assets/media/rinjani-flyover.mp4",
                poster: "assets/media/rinjani-flyover-poster.webp",
                label: "Route flyover, North Lombok Regency Trail Run",
                description: "COROS flyover of the full route through Mount Rinjani National Park.",
            },
            Slide::Image {
                src: "assets/media/rinjani.webp",
                alt: "COROS summary for the North Lombok Regency Trail Run",
                description: "COROS summary. 61.27 km, 19:47:29 activity time, 5,287 m elevation gain.",
            },
        ],
        gpx: Some((
            "assets/gpx/north-lombok-trail-run-2026-05-02.gpx",
            "14.3 MB",
        )),
    },
    RunPost {
        title: "CULTRA 60 km",
        place: "Cameron Highlands",
        date: "Jul 18, 2026",
        stats: &[
            ("58.65 km", "distance"),
            ("9:21:10", "time"),
            ("2,438 m", "gain"),
        ],
        note: "Highest placing Singaporean in the 60 km event.",
        slides: &[
            Slide::Video {
                src: "assets/media/cameron-flyover.mp4",
                poster: "assets/media/cameron-flyover-poster.webp",
                label: "Route flyover, CULTRA 60 km",
                description: "COROS flyover of the full route around Brinchang and Ladang Boh.",
            },
            Slide::Image {
                src: "assets/media/cameron.webp",
                alt: "COROS summary for CULTRA 60 km in Cameron Highlands",
                description: "COROS summary. 58.65 km, 9:21:10 activity time, 2,438 m elevation gain.",
            },
        ],
        gpx: Some((
            "assets/gpx/cameron-highlands-trail-run-2026-07-18.gpx",
            "9.5 MB",
        )),
    },
    RunPost {
        title: "202.54 km around Singapore",
        place: "Singapore",
        date: "Aug 21, 2026",
        stats: &[
            ("202.54 km", "distance"),
            ("46:46:23", "time"),
            ("$850", "for SHINE"),
        ],
        note: "One loop of the island, raising $850 for SHINE.",
        slides: &[
            Slide::Video {
                src: "assets/media/sg202-flyover.mp4",
                poster: "assets/media/sg202-flyover-poster.webp",
                label: "Route flyover, 202.54 km around Singapore",
                description: "COROS flyover of the full 202.54 km loop around Singapore.",
            },
            Slide::Image {
                src: "assets/media/sg202.webp",
                alt: "COROS summary for the 202.54 km Singapore run",
                description: "COROS summary. 202.54 km in 46:46:23 activity time.",
            },
        ],
        gpx: Some(("assets/gpx/singapore-202km-run-2026-08-21.gpx", "41.1 MB")),
    },
    RunPost {
        title: "BUS Backyard Ultra",
        place: "Singapore",
        date: "Aug 7, 2026",
        stats: &[("83.52 km", "distance"), ("12", "loops"), ("3rd", "place")],
        note: "Third place in a backyard format race by One More Lap.",
        slides: &[
            Slide::Video {
                src: "assets/media/run83-flyover.mp4",
                poster: "assets/media/run83-flyover-poster.webp",
                label: "Route flyover, BUS Backyard Ultra",
                description: "COROS flyover of the full 83.52 km on the watch.",
            },
            Slide::Image {
                src: "assets/media/run83.webp",
                alt: "Madrid running on grass with a COROS summary overlay",
                description: "Photo with the COROS summary. 83.52 km in 13:07:59.",
            },
            Slide::Image {
                src: "assets/media/bus.webp",
                alt: "Madrid and another runner holding a box at the Backyard Ultra Simulator",
                description: "At the Backyard Ultra Simulator by One More Lap.",
            },
        ],
        gpx: None,
    },
];

pub const EMAIL: &str = "e1398088@u.nus.edu";

pub struct Document {
    pub title: &'static str,
    pub detail: &'static str,
    pub href: &'static str,
    pub meta: &'static str,
}

pub const DOCUMENTS: [Document; 2] = [
    Document {
        title: "Resume",
        detail: "One page, Sep 2026",
        href: "assets/docs/madrid-lim-resume.pdf",
        meta: "PDF \u{b7} 670 KB",
    },
    Document {
        title: "NUS grades",
        detail: "Unofficial, GPA 4.39",
        href: "assets/docs/madrid-lim-nus-transcript.pdf",
        meta: "PDF \u{b7} 420 KB",
    },
];

pub const CONTACTS: [(&str, &str); 3] = [
    ("GitHub", "https://github.com/StarlightsJourney"),
    (
        "LinkedIn",
        "https://www.linkedin.com/in/madrid-lim-8264082a2/",
    ),
    ("Instagram", "https://www.instagram.com/grow.madrid/"),
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;

    use super::{CONTACTS, DOCUMENTS, PROJECTS, RUNS, Slide};

    fn asset_paths() -> Vec<&'static str> {
        let mut paths = PROJECTS
            .iter()
            .filter_map(|project| project.image)
            .collect::<Vec<_>>();
        for run in &RUNS {
            for slide in run.slides {
                match slide {
                    Slide::Video { src, poster, .. } => paths.extend([*src, *poster]),
                    Slide::Image { src, .. } => paths.push(src),
                }
            }
            if let Some((href, _)) = run.gpx {
                paths.push(href);
            }
        }
        paths.extend(DOCUMENTS.iter().map(|document| document.href));
        paths
    }

    fn visible_text() -> Vec<&'static str> {
        let mut text = Vec::new();
        for project in &PROJECTS {
            text.extend([
                project.name,
                project.category,
                project.role,
                project.summary,
                project.problem,
                project.built,
                project.stack,
                project.status,
            ]);
        }
        for run in &RUNS {
            text.extend([run.title, run.place, run.date, run.note]);
            for slide in run.slides {
                match slide {
                    Slide::Video {
                        label, description, ..
                    } => text.extend([*label, *description]),
                    Slide::Image {
                        alt, description, ..
                    } => text.extend([*alt, *description]),
                }
            }
        }
        text
    }

    #[test]
    fn project_names_are_nonempty_and_unique() {
        let mut names = HashSet::new();
        for project in &PROJECTS {
            assert!(!project.name.trim().is_empty());
            assert!(
                names.insert(project.name),
                "duplicate project: {}",
                project.name
            );
        }
    }

    #[test]
    fn links_use_only_https() {
        let project_links = PROJECTS
            .iter()
            .flat_map(|project| project.links.iter().map(|(_, href)| *href));
        let contact_links = CONTACTS.iter().map(|(_, href)| *href);
        for href in project_links.chain(contact_links) {
            assert!(
                href.starts_with("https://"),
                "unsupported link scheme: {href}"
            );
        }
    }

    #[test]
    fn referenced_assets_exist() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for path in asset_paths() {
            assert!(!path.starts_with('/'), "asset must be relative: {path}");
            assert!(root.join(path).is_file(), "missing asset: {path}");
        }
    }

    #[test]
    fn visible_copy_has_no_dashes() {
        for text in visible_text() {
            assert!(
                !text.contains('\u{2013}') && !text.contains('\u{2014}'),
                "dash in copy: {text}"
            );
        }
    }
}
