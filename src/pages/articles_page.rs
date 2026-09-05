use crate::markdown_management::{ArticleWithMetadata, fetch_standalone_articles};
use crate::shared::cover::CoverArt;
use dioxus::prelude::*;

const ARTICLES_PER_PAGE: usize = 12;

#[component]
pub fn ArticlesPage() -> Element {
    let mut current_page = use_signal(|| 1usize);

    let articles_data = use_resource(move || async move {
        let page = *current_page.read();
        fetch_standalone_articles(page, ARTICLES_PER_PAGE).await.ok()
    });

    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14",

                header { class: "mb-10 max-w-3xl", "data-reveal": "true",
                    p { class: "eyebrow mb-3", "Writing" }
                    h1 { class: "text-4xl md:text-5xl font-semibold mb-4 text-gradient animate-gradient-pan",
                        "Standalone articles"
                    }
                    p { class: "text-lg text-base-content/70 leading-relaxed",
                        "One-off pieces that don't belong to a series."
                    }
                }

                {
                    match articles_data.read().as_ref() {
                        Some(Some(data)) => {
                            if data.articles.is_empty() {
                                rsx! {
                                    EmptyState {
                                        message: if data.page > 1 {
                                            "Nothing on this page."
                                        } else {
                                            "No standalone articles yet."
                                        },
                                    }
                                }
                            } else {
                                let total = data.total_count;
                                let page = data.page;
                                let total_pages = data.total_pages;
                                rsx! {
                                    p { class: "eyebrow mb-6", "{total} article(s)" }
                                    div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                                        for (i , article) in data.articles.iter().enumerate() {
                                            ArticleCard {
                                                key: "{article.metadata.path}",
                                                article: article.clone(),
                                                index: i,
                                            }
                                        }
                                    }
                                    if total_pages > 1 {
                                        Pagination {
                                            current_page: page,
                                            total_pages,
                                            on_page_change: move |p| current_page.set(p),
                                        }
                                    }
                                }
                            }
                        }
                        Some(None) => rsx! {
                            EmptyState { message: "Could not load articles." }
                        },
                        None => rsx! {
                            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6",
                                for i in 0..6 {
                                    div { key: "{i}", class: "skeleton-shimmer h-80" }
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
fn EmptyState(message: String) -> Element {
    rsx! {
        div { class: "surface p-16 text-center", "data-reveal": "true",
            p { class: "text-base-content/50", "{message}" }
        }
    }
}

#[component]
fn ArticleCard(article: ArticleWithMetadata, index: usize) -> Element {
    let meta = article.toml_metadata.as_ref();
    let thumbnail = meta.and_then(|m| m.thumbnail.clone());
    let date = meta.and_then(|m| m.date.clone());
    let reading_time = meta.and_then(|m| m.reading_time.clone());
    let category = meta.and_then(|m| m.category.clone());
    let summary = meta.and_then(|m| m.summary.clone());
    let topics: Vec<String> = meta.map(|m| m.topics.clone()).unwrap_or_default();
    let href = format!("/article/{}", article.metadata.path.trim_end_matches(".md"));
    let delay = (index % 6) * 60;

    rsx! {
        div { "data-reveal": "true", style: "--reveal-delay: {delay}ms", class: "h-full",
            Link {
                to: href,
                class: "surface card-interactive card-spotlight group flex flex-col h-full p-0 overflow-hidden",

                // ---- cover ----
                div { class: "relative aspect-[16/9] overflow-hidden border-b border-[var(--hairline)]",
                    if let Some(thumb) = thumbnail {
                        img {
                            src: "{thumb}",
                            alt: "",
                            class: "absolute inset-0 w-full h-full object-cover transition-transform duration-[1100ms] ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-105",
                        }
                    } else {
                        CoverArt { seed: "{article.metadata.title}", accent: "var(--color-primary)" }
                    }
                    if let Some(cat) = category {
                        span { class: "absolute top-3 left-3 chip backdrop-blur-md", "{cat}" }
                    }
                }

                // ---- text ----
                div { class: "p-5 flex flex-col flex-1 relative z-[2]",
                    h3 { class: "text-lg font-semibold leading-snug mb-2.5 line-clamp-2 transition-colors duration-300 group-hover:text-primary",
                        "{article.metadata.title}"
                    }

                    if let Some(text) = summary {
                        p { class: "text-sm text-base-content/60 leading-relaxed line-clamp-3 mb-4 flex-1",
                            "{text}"
                        }
                    }

                    if !topics.is_empty() {
                        div { class: "flex flex-wrap gap-1.5 mb-4",
                            for topic in topics.iter().take(3) {
                                span { class: "chip", "{topic}" }
                            }
                        }
                    }

                    div { class: "mt-auto pt-3 border-t border-[var(--hairline)] flex items-center justify-between gap-2",
                        span { class: "eyebrow text-[0.58rem]",
                            {date.unwrap_or_default()}
                        }
                        span { class: "eyebrow text-[0.58rem]",
                            {reading_time.unwrap_or_default()}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Pagination(
    current_page: usize,
    total_pages: usize,
    on_page_change: EventHandler<usize>,
) -> Element {
    let start_page = if current_page <= 3 { 1 } else { current_page - 2 };
    let end_page = (start_page + 4).min(total_pages);

    rsx! {
        nav { class: "flex justify-center items-center gap-1.5 pt-12", aria_label: "Pagination",

            button {
                class: "pill pill--off disabled:opacity-35 disabled:pointer-events-none",
                disabled: current_page == 1,
                onclick: move |_| {
                    if current_page > 1 {
                        on_page_change.call(current_page - 1);
                    }
                },
                aria_label: "Previous page",
                "←"
            }

            if start_page > 1 {
                button { class: "pill pill--off", onclick: move |_| on_page_change.call(1), "1" }
                if start_page > 2 {
                    span { class: "px-1 text-base-content/35", "…" }
                }
            }

            for page in start_page..=end_page {
                button {
                    key: "{page}",
                    class: if page == current_page { "pill pill--on" } else { "pill pill--off" },
                    aria_current: if page == current_page { "page" } else { "false" },
                    onclick: move |_| on_page_change.call(page),
                    "{page}"
                }
            }

            if end_page < total_pages {
                if end_page + 1 < total_pages {
                    span { class: "px-1 text-base-content/35", "…" }
                }
                button {
                    class: "pill pill--off",
                    onclick: move |_| on_page_change.call(total_pages),
                    "{total_pages}"
                }
            }

            button {
                class: "pill pill--off disabled:opacity-35 disabled:pointer-events-none",
                disabled: current_page == total_pages,
                onclick: move |_| {
                    if current_page < total_pages {
                        on_page_change.call(current_page + 1);
                    }
                },
                aria_label: "Next page",
                "→"
            }
        }
    }
}
