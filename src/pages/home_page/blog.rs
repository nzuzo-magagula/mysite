use dioxus::prelude::*;

use crate::markdown_management::{ArticleWithMetadata, fetch_home_page_data_with_metadata};
use crate::shared::cover::CoverArt;

#[component]
pub fn Blogs() -> Element {
    let home_data = use_resource(|| async move { fetch_home_page_data_with_metadata().await.ok() });

    rsx! {
        div { class: "space-y-14",

            // ---- Featured ----
            section {
                SectionHeading { accent_class: "bg-primary", title: "Latest deep dive" }

                {
                    match home_data.read().as_ref() {
                        Some(Some(data)) => {
                            if let Some(article) = &data.first_article {
                                rsx! {
                                    FeaturedArticle { article: article.clone() }
                                }
                            } else {
                                rsx! {
                                    div { class: "surface p-12 text-center text-base-content/50",
                                        "No articles yet."
                                    }
                                }
                            }
                        }
                        _ => rsx! {
                            div { class: "skeleton-shimmer h-64" }
                        },
                    }
                }
            }

            // ---- Recent ----
            section {
                SectionHeading { accent_class: "bg-secondary", title: "Recent writing" }

                div { class: "grid gap-5 md:grid-cols-2",
                    {
                        match home_data.read().as_ref() {
                            Some(Some(data)) => {
                                let recent: Vec<ArticleWithMetadata> = data
                                    .recent_articles
                                    .iter()
                                    .skip(1)
                                    .take(4)
                                    .cloned()
                                    .collect();
                                rsx! {
                                    for (i , article) in recent.iter().enumerate() {
                                        ArticleCard { key: "{article.metadata.path}", article: article.clone(), index: i }
                                    }
                                    Link {
                                        to: "/articles",
                                        class: "md:col-span-2 group py-5 rounded-[var(--radius-box)] border border-dashed border-[var(--hairline-strong)] flex items-center justify-center gap-2 text-primary font-semibold hover:bg-primary/5 hover:border-primary/50 transition-all duration-400",
                                        "data-reveal": "true",
                                        "Every article"
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
                            _ => rsx! {
                                for i in 0..4 {
                                    div { key: "{i}", class: "skeleton-shimmer h-44" }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SectionHeading(accent_class: String, title: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-3 mb-6", "data-reveal": "true",
            span { class: "h-5 w-[3px] rounded-full {accent_class}" }
            h2 { class: "text-xl font-semibold", "{title}" }
            span { class: "flex-1 h-px bg-[var(--hairline)]" }
        }
    }
}

#[component]
fn FeaturedArticle(article: ArticleWithMetadata) -> Element {
    let meta = article.toml_metadata.as_ref();
    let thumbnail = meta.and_then(|m| m.thumbnail.clone());
    let date = meta.and_then(|m| m.date.clone()).unwrap_or_default();
    let category = meta
        .and_then(|m| m.category.clone())
        .unwrap_or_else(|| "Deep dive".to_string());
    let read_time = meta.and_then(|m| m.reading_time.clone());
    let summary = meta.and_then(|m| m.summary.clone()).unwrap_or_else(|| {
        article.content.chars().take(220).collect::<String>() + "…"
    });
    let href = format!("/article/{}", article.metadata.path.trim_end_matches(".md"));

    rsx! {
        div { "data-reveal": "true",
            Link {
                to: href,
                class: "surface card-interactive card-spotlight group block overflow-hidden p-0",

                div { class: "md:flex",
                    // ---- cover ----
                    div { class: "md:w-2/5 relative h-52 md:h-auto md:min-h-[15rem] overflow-hidden border-b md:border-b-0 md:border-r border-[var(--hairline)]",
                        if let Some(src) = thumbnail {
                            img {
                                src: "{src}",
                                alt: "",
                                class: "absolute inset-0 w-full h-full object-cover transition-transform duration-[1100ms] ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-105",
                            }
                        } else {
                            CoverArt { seed: "{article.metadata.title}", accent: "var(--color-primary)" }
                        }
                    }

                    // ---- text ----
                    div { class: "p-7 md:p-8 md:w-3/5 flex flex-col justify-center relative z-[2]",
                        div { class: "flex flex-wrap items-center gap-2 mb-3",
                            span { class: "chip", style: "color: var(--color-primary);", "{category}" }
                            if !date.is_empty() {
                                span { class: "chip", "{date}" }
                            }
                            if let Some(rt) = read_time {
                                span { class: "chip", "{rt}" }
                            }
                        }

                        h3 { class: "text-2xl md:text-[1.7rem] leading-snug font-semibold mb-3 transition-colors duration-300 group-hover:text-primary",
                            "{article.metadata.title}"
                        }

                        p { class: "text-base-content/65 leading-relaxed line-clamp-3 mb-5", "{summary}" }

                        span { class: "inline-flex items-center gap-1.5 text-primary font-semibold text-sm",
                            "Read article"
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
}

#[component]
fn ArticleCard(article: ArticleWithMetadata, index: usize) -> Element {
    let meta = article.toml_metadata.as_ref();
    let date = meta.and_then(|m| m.date.clone()).unwrap_or_default();
    let category = meta
        .and_then(|m| m.category.clone())
        .unwrap_or_else(|| "Article".to_string());
    let read_time = meta
        .and_then(|m| m.reading_time.clone())
        .unwrap_or_else(|| "5 min read".to_string());
    let summary = meta.and_then(|m| m.summary.clone()).unwrap_or_else(|| {
        article.content.chars().take(150).collect::<String>() + "…"
    });
    let href = format!("/article/{}", article.metadata.path.trim_end_matches(".md"));
    let delay = (index % 4) * 70;

    rsx! {
        div { "data-reveal": "true", style: "--reveal-delay: {delay}ms", class: "h-full",
            Link {
                to: href,
                class: "surface card-interactive card-spotlight group flex flex-col h-full p-5",

                div { class: "flex items-center justify-between gap-2 mb-3 relative z-[2]",
                    span { class: "chip", "{category}" }
                    span { class: "eyebrow text-[0.58rem]", "{read_time}" }
                }

                h3 { class: "text-lg font-semibold leading-snug mb-2 line-clamp-2 transition-colors duration-300 group-hover:text-primary relative z-[2]",
                    "{article.metadata.title}"
                }

                p { class: "text-sm text-base-content/60 leading-relaxed line-clamp-3 flex-1 relative z-[2]",
                    "{summary}"
                }

                if !date.is_empty() {
                    p { class: "eyebrow text-[0.58rem] mt-4 pt-3 border-t border-[var(--hairline)] relative z-[2]",
                        "{date}"
                    }
                }
            }
        }
    }
}
