use crate::markdown_management::github::{GitHubAccountType, GitHubRepo, fetch_github_repos};
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let repos = use_resource(|| async move {
        fetch_github_repos(GitHubAccountType::User, "newsnet-africa".to_string())
            .await
            .unwrap_or_default()
    });

    rsx! {
        div { class: "space-y-4", "data-reveal": "right",

            // ---- repositories ----
            div { class: "surface p-5",
                h3 { class: "eyebrow mb-4 flex items-center gap-2",
                    span { class: "relative flex h-1.5 w-1.5",
                        span { class: "absolute inline-flex h-full w-full rounded-full bg-success opacity-75 animate-ping" }
                        span { class: "relative inline-flex rounded-full h-1.5 w-1.5 bg-success" }
                    }
                    "Currently building"
                }

                div { class: "space-y-1 -mx-2",
                    match repos.value().as_ref() {
                        Some(repo_list) => rsx! {
                            if repo_list.is_empty() {
                                p { class: "px-2 py-4 text-sm text-base-content/45", "No public repositories found." }
                            } else {
                                for (i , repo) in repo_list.iter().take(5).enumerate() {
                                    ProjectItem { key: "{repo.html_url}", repo: repo.clone(), index: i }
                                }
                            }
                        },
                        None => rsx! {
                            for i in 0..3 {
                                div { key: "{i}", class: "skeleton-shimmer h-14 mx-2 mb-1" }
                            }
                        },
                    }
                }

                a {
                    href: "https://github.com/nzuzo-newsnet",
                    class: "group mt-5 pt-4 border-t border-[var(--hairline)] flex items-center gap-1.5 text-sm font-semibold text-primary transition-colors duration-300",
                    "All repositories"
                    svg {
                        class: "w-3.5 h-3.5 transition-transform duration-300 group-hover:translate-x-1",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 5l7 7-7 7" }
                    }
                }
            }

            // ---- contact ----
            div {
                class: "surface card-spotlight relative overflow-hidden p-5",
                style: "background-image: linear-gradient(140deg, color-mix(in oklab, var(--color-primary) 10%, transparent), color-mix(in oklab, var(--color-accent) 7%, transparent));",

                h4 { class: "eyebrow mb-2 relative z-[2]", "Get in touch" }
                p { class: "text-sm text-base-content/70 leading-relaxed mb-4 relative z-[2]",
                    "Working on something in Rust, databases or distributed systems? I'd like to hear about it."
                }

                div { class: "grid grid-cols-2 gap-2 relative z-[2]",
                    a {
                        href: "https://github.com/nzuzo-newsnet",
                        class: "py-2 text-center text-sm font-semibold rounded-[var(--radius-field)] bg-primary text-primary-content hover:brightness-110 transition-all duration-300 hover:-translate-y-0.5",
                        "GitHub"
                    }
                    a {
                        href: "mailto:dev@nzuzomagagula.online",
                        class: "py-2 text-center text-sm font-semibold rounded-[var(--radius-field)] border border-[var(--hairline-strong)] hover:border-primary/50 hover:bg-base-100/60 transition-all duration-300 hover:-translate-y-0.5",
                        "Email"
                    }
                }

                p { class: "mt-3 text-[0.7rem] font-mono text-base-content/45 relative z-[2] text-center",
                    "dev@nzuzomagagula.online"
                }
            }
        }
    }
}

#[component]
fn ProjectItem(repo: GitHubRepo, index: usize) -> Element {
    rsx! {
        a {
            href: "{repo.html_url}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "flex items-start gap-2.5 p-2 rounded-[var(--radius-field)] transition-colors duration-300 group hover:bg-base-200/60",
            "data-reveal": "true",
            style: "--reveal-delay: {index * 60}ms",

            div { class: "mt-0.5 p-1.5 rounded-md bg-primary/10 text-primary transition-transform duration-300 group-hover:scale-110",
                svg {
                    class: "w-3.5 h-3.5",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
                    }
                }
            }

            div { class: "flex-1 min-w-0",
                div { class: "flex items-center justify-between gap-2",
                    h4 { class: "font-semibold text-sm truncate transition-colors duration-300 group-hover:text-primary",
                        "{repo.name}"
                    }
                    if repo.stargazers_count > 0 {
                        span { class: "text-[0.65rem] font-mono text-base-content/40 flex-shrink-0",
                            "★ {repo.stargazers_count}"
                        }
                    }
                }
                if let Some(desc) = &repo.description {
                    p { class: "text-xs text-base-content/50 truncate mt-0.5", "{desc}" }
                }
                if let Some(lang) = &repo.language {
                    span { class: "chip mt-1.5", "{lang}" }
                }
            }
        }
    }
}
