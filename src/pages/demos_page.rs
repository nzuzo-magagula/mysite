use crate::Route;
use crate::shared::cover::NodeArt;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Demo {
    title: &'static str,
    description: &'static str,
    category: DemoCategory,
    stack: &'static [&'static str],
    link: Option<Route>,
    /// Seeds the generated cover artwork so every card looks distinct.
    art: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoCategory {
    Interactive,
    Systems,
    Data,
}

impl DemoCategory {
    const ALL: [DemoCategory; 3] = [
        DemoCategory::Interactive,
        DemoCategory::Systems,
        DemoCategory::Data,
    ];

    fn as_str(&self) -> &'static str {
        match self {
            DemoCategory::Interactive => "Interactive",
            DemoCategory::Systems => "Systems",
            DemoCategory::Data => "Data",
        }
    }

    /// Accent hue used by the card artwork and badge.
    fn hue(&self) -> &'static str {
        match self {
            DemoCategory::Interactive => "var(--color-primary)",
            DemoCategory::Systems => "var(--color-secondary)",
            DemoCategory::Data => "var(--color-accent)",
        }
    }
}

const DEMOS: &[Demo] = &[
    Demo {
        title: "Tiny Neural Network",
        description: "A multilayer perceptron trained live in the browser — backpropagation written by hand in Python, running on Pyodide in a Web Worker, with the decision boundary redrawn every couple of epochs.",
        category: DemoCategory::Interactive,
        stack: &["Svelte 5", "Python", "Pyodide", "WebAssembly"],
        link: Some(Route::NeuralNet {}),
        art: 3,
    },
    Demo {
        title: "Algorithm Visualiser",
        description: "Sorting and pathfinding stepped one operation at a time, with the pseudocode line highlighted as it executes.",
        category: DemoCategory::Interactive,
        stack: &["React", "Vite", "Generators"],
        link: Some(Route::AlgoVis {}),
        art: 7,
    },
    Demo {
        title: "Netabase Store",
        description: "An embedded key–value store with derive-macro schemas, automatic secondary indexes and a zero-copy read path. The write-up series goes through the internals.",
        category: DemoCategory::Systems,
        stack: &["Rust", "Proc macros", "bincode"],
        link: None,
        art: 11,
    },
    Demo {
        title: "This Site",
        description: "A Rust web app compiled to WebAssembly: Dioxus for the UI, a custom Markdown pipeline for the articles, and a file watcher that hot-reloads content in development.",
        category: DemoCategory::Systems,
        stack: &["Rust", "Dioxus", "WASM", "Tailwind"],
        link: None,
        art: 19,
    },
    Demo {
        title: "News Analytics",
        description: "Tracking how stories propagate across African media outlets — ingestion, deduplication and trend surfacing.",
        category: DemoCategory::Data,
        stack: &["Python", "Postgres"],
        link: None,
        art: 23,
    },
];

#[component]
pub fn DemosPage() -> Element {
    let mut selected = use_signal(|| None::<DemoCategory>);

    let filtered: Vec<&'static Demo> = DEMOS
        .iter()
        .filter(|d| selected.read().map_or(true, |c| d.category == c))
        .collect();

    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14",

                // ---- header ----
                header { class: "mb-10 max-w-3xl", "data-reveal": "true",
                    p { class: "eyebrow mb-3", "Demos" }
                    h1 { class: "text-4xl md:text-5xl font-semibold mb-4 text-gradient animate-gradient-pan",
                        "Things you can poke at"
                    }
                    p { class: "text-lg text-base-content/70 leading-relaxed",
                        "Small builds that run in the page rather than in a screenshot. Each one is a
                         self-contained app embedded in the site."
                    }
                }

                // ---- filters ----
                div { class: "flex gap-2 mb-10 overflow-x-auto pb-1 delay-step-1", "data-reveal": "true",
                    button {
                        class: if selected.read().is_none() { "pill pill--on" } else { "pill pill--off" },
                        onclick: move |_| selected.set(None),
                        "All"
                    }
                    for category in DemoCategory::ALL {
                        button {
                            class: if *selected.read() == Some(category) { "pill pill--on" } else { "pill pill--off" },
                            onclick: move |_| selected.set(Some(category)),
                            "{category.as_str()}"
                        }
                    }
                }

                // ---- grid ----
                div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                    for (i , demo) in filtered.iter().enumerate() {
                        DemoCard { key: "{demo.title}", demo: (*demo).clone(), index: i }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoCard(demo: Demo, index: usize) -> Element {
    let delay = (index % 6) * 70;

    let body = rsx! {
        // ---- generated cover ----
        div {
            class: "relative h-40 overflow-hidden border-b border-[var(--hairline)]",
            style: "background: color-mix(in oklab, {demo.category.hue()} 9%, transparent);",
            NodeArt { seed: demo.art, accent: demo.category.hue() }
            span {
                class: "absolute top-3 right-3 chip backdrop-blur-md",
                style: "color: {demo.category.hue()}; border-color: color-mix(in oklab, {demo.category.hue()} 35%, transparent);",
                "{demo.category.as_str()}"
            }
        }

        // ---- text ----
        div { class: "p-6 flex flex-col flex-1 relative z-[2]",
            h3 { class: "text-xl font-semibold mb-2.5 transition-colors duration-300 group-hover:text-primary",
                "{demo.title}"
            }
            p { class: "text-sm text-base-content/65 leading-relaxed mb-5 flex-1",
                "{demo.description}"
            }

            div { class: "flex flex-wrap gap-1.5 mb-5",
                for tech in demo.stack.iter() {
                    span { class: "chip", "{tech}" }
                }
            }

            div { class: "mt-auto pt-4 border-t border-[var(--hairline)]",
                if demo.link.is_some() {
                    span { class: "inline-flex items-center gap-1.5 text-primary font-semibold text-sm",
                        "Open demo"
                        svg {
                            class: "w-4 h-4 transition-transform duration-300 group-hover:translate-x-1",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 5l7 7-7 7" }
                        }
                    }
                } else {
                    span { class: "eyebrow", "Write-up in progress" }
                }
            }
        }
    };

    let classes = "surface card-interactive card-spotlight group flex flex-col h-full p-0 overflow-hidden";

    rsx! {
        div { "data-reveal": "true", style: "--reveal-delay: {delay}ms", class: "h-full",
            if let Some(route) = demo.link.clone() {
                Link { to: route, class: "{classes}", {body} }
            } else {
                article { class: "{classes} cursor-default", {body} }
            }
        }
    }
}
