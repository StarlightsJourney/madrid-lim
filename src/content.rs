#[allow(dead_code)]
pub struct Project {
    pub name: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
    pub problem: &'static str,
    pub built: &'static str,
    pub why: &'static str,
    pub stack: &'static str,
    pub href: Option<&'static str>,
    pub status: &'static str,
}

pub const PROJECTS: [Project; 6] = [
    Project {
        name: "Ka-teng",
        category: "family lineage",
        summary: "A family tree that stays readable as families grow.",
        problem: "Family history is hard to navigate in spreadsheets and chat threads.",
        built: "An open source lineage visualiser with relationships as the core interaction.",
        why: "Makes family history easier to explore and pass on.",
        stack: "web, graph data, open source",
        href: Some("https://github.com/StarlightsJourney/Ka-teng"),
        status: "view on GitHub",
    },
    Project {
        name: "HillGPX",
        category: "maps and trails",
        summary: "A community map for finding elevated trails around the world.",
        problem: "Route platforms make it difficult to discover hills by terrain.",
        built: "Search, map and trail browsing around elevation first discovery.",
        why: "Helps runners find useful terrain before travelling there.",
        stack: "maps, geospatial data, community photos",
        href: Some("https://github.com/StarlightsJourney/HillGPX"),
        status: "view on GitHub",
    },
    Project {
        name: "Sheng",
        category: "language learning",
        summary: "Learn from the videos you already watch.",
        problem: "Looking up a line and turning it into something reviewable breaks the flow.",
        built: "Capture, lookup, save and review in one desktop workflow.",
        why: "Keeps immersion connected to deliberate practice.",
        stack: "rust, tauri, svelte, sqlite",
        href: None,
        status: "private / in development",
    },
    Project {
        name: "NoSleepMenuBar",
        category: "macOS utility",
        summary: "One click keeps a Mac awake.",
        problem: "A tiny system task should not need a large app.",
        built: "A native menu bar utility with a focused single purpose control.",
        why: "Small software can still remove daily friction.",
        stack: "swift, macOS",
        href: Some("https://github.com/StarlightsJourney/NoSleepMenuBar"),
        status: "view on GitHub",
    },
    Project {
        name: "NUS Timetable Optimizer",
        category: "constraint scheduling",
        summary: "Rank timetables by the tradeoffs students care about.",
        problem: "A clash free timetable can still have bad gaps, early lessons and long walks.",
        built: "Constraint filtering and weighted scoring over module and venue data.",
        why: "Returns feasible schedules ordered by personal preference.",
        stack: "python, nusmods, telegram",
        href: None,
        status: "private / in development",
    },
    Project {
        name: "HejAmigo",
        category: "local marketplace",
        summary: "A Denmark first marketplace for local errands.",
        problem: "Small local tasks need a simple way to match requesters and helpers.",
        built: "Posting, offers, accepted task messaging, profiles and reviews.",
        why: "Creates a clear end to end task flow.",
        stack: "expo, supabase, maps",
        href: None,
        status: "private / in development",
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

    use super::{CONTACTS, PROJECTS};

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
        let project_links = PROJECTS.iter().filter_map(|project| project.href);
        let contact_links = CONTACTS.iter().map(|(_, href)| *href);
        for href in project_links.chain(contact_links) {
            assert!(
                href.starts_with("https://"),
                "unsupported link scheme: {href}"
            );
        }
    }
}
