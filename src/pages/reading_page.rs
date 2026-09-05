use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ReadingItem {
    title: &'static str,
    author: &'static str,
    description: &'static str,
    status: ReadingStatus,
    link: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReadingStatus {
    CurrentlyReading,
    Read,
    WantToRead,
}

impl ReadingStatus {
    const ALL: [ReadingStatus; 3] = [
        ReadingStatus::CurrentlyReading,
        ReadingStatus::Read,
        ReadingStatus::WantToRead,
    ];

    fn as_str(&self) -> &'static str {
        match self {
            ReadingStatus::CurrentlyReading => "Reading",
            ReadingStatus::Read => "Finished",
            ReadingStatus::WantToRead => "Queued",
        }
    }

    /// Colour of the spine marker on each row.
    fn tint(&self) -> &'static str {
        match self {
            ReadingStatus::CurrentlyReading => "var(--color-primary)",
            ReadingStatus::Read => "var(--color-success)",
            ReadingStatus::WantToRead => "var(--color-base-300)",
        }
    }
}

const READING_LIST: &[ReadingItem] = &[
    ReadingItem {
        title: "Designing Data-Intensive Applications",
        author: "Martin Kleppmann",
        description: "The reference for how storage, replication and consensus actually fit together. Worth rereading whenever a new system design question comes up.",
        status: ReadingStatus::CurrentlyReading,
        link: None,
    },
    ReadingItem {
        title: "Release It!",
        author: "Michael T. Nygard",
        description: "Failure modes of production systems, and the patterns — bulkheads, circuit breakers, timeouts — that contain them.",
        status: ReadingStatus::CurrentlyReading,
        link: None,
    },
    ReadingItem {
        title: "The Rust Programming Language",
        author: "Steve Klabnik, Carol Nichols",
        description: "The official book. The ownership chapters are the ones that make everything afterwards click.",
        status: ReadingStatus::Read,
        link: Some("https://doc.rust-lang.org/book/"),
    },
    ReadingItem {
        title: "Clean Architecture",
        author: "Robert C. Martin",
        description: "Useful on dependency direction and boundaries, even where you disagree with the prescriptions.",
        status: ReadingStatus::Read,
        link: None,
    },
    ReadingItem {
        title: "The Art of Doing Science and Engineering",
        author: "Richard Hamming",
        description: "Less about technique than about choosing which problems are worth years of your attention.",
        status: ReadingStatus::WantToRead,
        link: None,
    },
];

#[component]
pub fn ReadingPage() -> Element {
    let mut selected = use_signal(|| None::<ReadingStatus>);

    let filtered: Vec<&'static ReadingItem> = READING_LIST
        .iter()
        .filter(|item| selected.read().map_or(true, |s| item.status == s))
        .collect();

    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-14",

                header { class: "mb-9", "data-reveal": "true",
                    p { class: "eyebrow mb-3", "Reading" }
                    h1 { class: "text-4xl md:text-5xl font-semibold mb-4 text-gradient animate-gradient-pan",
                        "What's on the desk"
                    }
                    p { class: "text-lg text-base-content/70 leading-relaxed",
                        "Books and papers in progress, finished, or waiting."
                    }
                }

                div { class: "flex gap-2 mb-8 flex-wrap delay-step-1", "data-reveal": "true",
                    button {
                        class: if selected.read().is_none() { "pill pill--on" } else { "pill pill--off" },
                        onclick: move |_| selected.set(None),
                        "All"
                    }
                    for status in ReadingStatus::ALL {
                        button {
                            class: if *selected.read() == Some(status) { "pill pill--on" } else { "pill pill--off" },
                            onclick: move |_| selected.set(Some(status)),
                            "{status.as_str()}"
                        }
                    }
                }

                div { class: "space-y-3",
                    for (i , item) in filtered.iter().enumerate() {
                        ReadingCard { key: "{item.title}", item: (*item).clone(), index: i }
                    }

                    if filtered.is_empty() {
                        div { class: "surface p-14 text-center",
                            p { class: "text-base-content/50", "Nothing in this list yet." }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ReadingCard(item: ReadingItem, index: usize) -> Element {
    let delay = (index % 6) * 60;

    rsx! {
        article {
            class: "surface card-interactive card-spotlight group relative flex gap-4 p-5 pl-6",
            "data-reveal": "true",
            style: "--reveal-delay: {delay}ms",

            // Spine marker, coloured by status.
            span {
                class: "absolute left-0 top-4 bottom-4 w-[3px] rounded-full transition-all duration-500 group-hover:top-2 group-hover:bottom-2",
                style: "background: {item.status.tint()};",
                aria_hidden: "true",
            }

            div { class: "flex-1 min-w-0 relative z-[2]",
                div { class: "flex flex-wrap items-start justify-between gap-3 mb-1.5",
                    h3 { class: "text-lg font-semibold leading-snug",
                        if let Some(link) = item.link {
                            a {
                                href: "{link}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "link-underline hover:text-primary transition-colors duration-300",
                                "{item.title}"
                            }
                        } else {
                            "{item.title}"
                        }
                    }
                    span {
                        class: "chip flex-shrink-0",
                        style: "color: {item.status.tint()}; border-color: color-mix(in oklab, {item.status.tint()} 40%, transparent);",
                        "{item.status.as_str()}"
                    }
                }

                p { class: "eyebrow text-[0.6rem] mb-3", "{item.author}" }
                p { class: "text-sm text-base-content/65 leading-relaxed", "{item.description}" }
            }
        }
    }
}
