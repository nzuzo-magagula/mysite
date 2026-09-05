use crate::markdown_management::{SeriesData, fetch_all_series};
use crate::shared::cover::CoverArt;
use dioxus::prelude::*;

#[component]
pub fn SeriesPage() -> Element {
    let series_data = use_resource(|| async move { fetch_all_series().await.ok() });

    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14",

                header { class: "mb-10 max-w-3xl", "data-reveal": "true",
                    p { class: "eyebrow mb-3", "Series" }
                    h1 { class: "text-4xl md:text-5xl font-semibold mb-4 text-gradient animate-gradient-pan",
                        "Written in parts"
                    }
                    p { class: "text-lg text-base-content/70 leading-relaxed",
                        "Longer subjects broken into a sequence you can read in order."
                    }
                }

                {
                    match series_data.read().as_ref() {
                        Some(Some(list)) => {
                            if list.is_empty() {
                                rsx! {
                                    div { class: "surface p-16 text-center", "data-reveal": "true",
                                        p { class: "text-base-content/50",
                                            "No series yet — they're built from folders inside the articles directory."
                                        }
                                    }
                                }
                            } else {
                                let count = list.len();
                                rsx! {
                                    p { class: "eyebrow mb-6", "{count} series" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                                        for (i , series) in list.iter().enumerate() {
                                            SeriesCard { key: "{series.name}", series: series.clone(), index: i }
                                        }
                                    }
                                }
                            }
                        }
                        Some(None) => rsx! {
                            div { class: "surface p-16 text-center",
                                p { class: "text-base-content/50", "Could not load series." }
                            }
                        },
                        None => rsx! {
                            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                                for i in 0..3 {
                                    div { key: "{i}", class: "skeleton-shimmer h-72" }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn SeriesCard(series: SeriesData, index: usize) -> Element {
    let count = series.articles.len();
    let label = if count == 1 { "part" } else { "parts" };
    let delay = (index % 6) * 60;

    rsx! {
        div { "data-reveal": "true", style: "--reveal-delay: {delay}ms", class: "h-full",
            Link {
                to: format!("/series/{}", series.name),
                class: "surface card-interactive card-spotlight group flex flex-col h-full p-0 overflow-hidden",

                div { class: "relative h-32 overflow-hidden border-b border-[var(--hairline)]",
                    CoverArt { seed: "{series.name}", accent: "var(--color-secondary)" }
                    span { class: "absolute top-3 right-3 chip backdrop-blur-md", "{count} {label}" }
                }

                div { class: "p-6 flex flex-col flex-1 relative z-[2]",
                    h3 { class: "text-xl font-semibold mb-3 transition-colors duration-300 group-hover:text-primary",
                        "{series.name}"
                    }

                    p { class: "text-sm text-base-content/60 leading-relaxed line-clamp-4 flex-1",
                        if let Some(ref summary) = series.short_summary {
                            "{summary}"
                        } else {
                            "A {count}-part series."
                        }
                    }

                    span { class: "mt-5 pt-4 border-t border-[var(--hairline)] inline-flex items-center gap-1.5 text-primary font-semibold text-sm",
                        "Start reading"
                        svg {
                            class: "w-4 h-4 transition-transform duration-300 group-hover:translate-x-1.5",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 5l7 7-7 7" }
                        }
                    }
                }
            }
        }
    }
}
