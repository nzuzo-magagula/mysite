use crate::markdown_management::{ArticleWithMetadata, fetch_series_by_name};
use dioxus::prelude::*;
use dioxus_markdown::Markdown;

#[component]
pub fn SeriesDetailPage(series_name: String) -> Element {
    let series_data = use_resource(move || {
        let name = series_name.clone();
        async move { fetch_series_by_name(name).await.ok() }
    });

    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12",

                Link {
                    to: "/series",
                    class: "group inline-flex items-center gap-1.5 mb-9 text-sm font-medium text-base-content/60 hover:text-primary transition-colors duration-300",
                    svg {
                        class: "w-4 h-4 transition-transform duration-300 group-hover:-translate-x-1",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15 19l-7-7 7-7" }
                    }
                    "All series"
                }

                {
                    match series_data.read().as_ref() {
                        Some(Some(series)) => {
                            let label = if series.total_articles == 1 { "part" } else { "parts" };
                            rsx! {
                                header { class: "mb-10", "data-reveal": "true",
                                    p { class: "eyebrow mb-3", "Series · {series.total_articles} {label}" }
                                    h1 { class: "text-4xl md:text-5xl font-semibold text-gradient animate-gradient-pan",
                                        "{series.name}"
                                    }
                                }
                                if let Some(ref long_summary) = series.long_summary {
                                    div {
                                        class: "surface p-7 md:p-9 mb-10",
                                        "data-reveal": "true",
                                        style: "--reveal-delay: 80ms",
                                        div { class: "prose max-w-none",
                                            Markdown { content: long_summary.clone() }
                                        }
                                    }
                                }
                                section {
                                    div { class: "flex items-center gap-3 mb-6", "data-reveal": "true",
                                        span { class: "h-5 w-[3px] rounded-full bg-primary" }
                                        h2 { class: "text-xl font-semibold", "In this series" }
                                        span { class: "flex-1 h-px bg-[var(--hairline)]" }
                                    }
                                    if series.articles.is_empty() {
                                        div { class: "surface p-14 text-center",
                                            p { class: "text-base-content/50", "Nothing published in this series yet." }
                                        }
                                    } else {
                                        // A numbered spine runs down the left of the list.
                                        ol { class: "relative space-y-3 pl-0",
                                            for (idx , article) in series.articles.iter().enumerate() {
                                                PartCard {
                                                    key: "{article.metadata.path}",
                                                    article: article.clone(),
                                                    index: idx + 1,
                                                    is_last: idx + 1 == series.articles.len(),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(None) => rsx! {
                            div { class: "surface p-14 text-center",
                                h2 { class: "text-xl font-semibold mb-2", "Series not found" }
                                p { class: "text-base-content/55 mb-6",
                                    "That series either moved or never existed."
                                }
                                Link { to: "/series", class: "btn-solid", "Back to all series" }
                            }
                        },
                        None => rsx! {
                            div { class: "space-y-3",
                                div { class: "skeleton-shimmer h-28 mb-8" }
                                for i in 0..4 {
                                    div { key: "{i}", class: "skeleton-shimmer h-24" }
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
fn PartCard(article: ArticleWithMetadata, index: usize, is_last: bool) -> Element {
    let meta = article.toml_metadata.as_ref();
    let date = meta.and_then(|m| m.date.clone());
    let reading_time = meta.and_then(|m| m.reading_time.clone());
    let summary = meta.and_then(|m| m.summary.clone());
    let href = format!("/article/{}", article.metadata.path.trim_end_matches(".md"));
    let delay = (index.saturating_sub(1) % 8) * 55;

    rsx! {
        li {
            class: "relative list-none",
            "data-reveal": "true",
            style: "--reveal-delay: {delay}ms",

            Link {
                to: href,
                class: "surface card-interactive card-spotlight group flex items-start gap-4 p-5",

                // Numbered marker, with a connector to the next part.
                div { class: "relative flex-shrink-0",
                    span { class: "flex items-center justify-center w-9 h-9 rounded-full font-mono text-sm font-bold bg-primary/12 text-primary border border-primary/25 transition-all duration-400 group-hover:bg-primary group-hover:text-primary-content group-hover:scale-110",
                        "{index}"
                    }
                    if !is_last {
                        span {
                            class: "absolute left-1/2 top-[calc(100%+0.35rem)] h-[calc(100%+0.5rem)] w-px -translate-x-1/2 bg-[var(--hairline)]",
                            aria_hidden: "true",
                        }
                    }
                }

                div { class: "flex-1 min-w-0 relative z-[2]",
                    h3 { class: "font-semibold text-lg leading-snug mb-1.5 transition-colors duration-300 group-hover:text-primary",
                        "{article.metadata.title}"
                    }

                    div { class: "flex flex-wrap gap-1.5 mb-2",
                        if let Some(d) = date {
                            span { class: "chip", "{d}" }
                        }
                        if let Some(rt) = reading_time {
                            span { class: "chip", "{rt}" }
                        }
                    }

                    if let Some(text) = summary {
                        p { class: "text-sm text-base-content/60 leading-relaxed line-clamp-2", "{text}" }
                    }
                }

                svg {
                    class: "flex-shrink-0 w-5 h-5 mt-2 text-base-content/30 transition-all duration-300 group-hover:text-primary group-hover:translate-x-1",
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
