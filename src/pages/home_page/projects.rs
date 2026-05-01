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
        div {
            class: "p-6 rounded-xl border border-base-300 bg-base-100 shadow-sm",
            h3 { class: "font-bold text-lg mb-4 flex items-center gap-2",
                svg { class: "w-5 h-5 text-primary", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" }
                }
                "Working on"
            }

            div {
                class: "space-y-4",
                match repos.value().as_ref() {
                    Some(repo_list) => rsx! {
                        if repo_list.is_empty() {
                            div { class: "text-center py-4 text-base-content/50 text-sm", "No public projects found." }
                        } else {
                            for repo in repo_list.iter().take(5) {
                                ProjectItem {
                                    repo: repo.clone()
                                }
                            }
                        }
                    },
                    None => rsx! {
                        for _ in 0..3 {
                            div { class: "h-16 bg-base-200 animate-pulse rounded-lg" }
                        }
                    }
                }
            }

            div {
                class: "mt-6 pt-4 border-t border-base-300",
                a {
                    href: "https://github.com/nzuzo-newsnet",
                    class: "text-sm text-primary hover:text-primary-focus flex items-center gap-1 font-medium transition-colors group",
                    "View all repositories"
                    svg { class: "w-4 h-4 group-hover:translate-x-1 transition-transform", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 5l7 7-7 7" }
                    }
                }
            }

            // Connect Section
            div {
                class: "mt-8 p-5 rounded-xl bg-gradient-to-br from-primary/10 to-secondary/10 border border-primary/20",
                h4 { class: "font-bold text-xs uppercase tracking-wide mb-2 opacity-70", "Connect" }
                p { class: "text-sm mb-4 text-base-content/70", "Interested in Rust or distributed systems? Let's chat." }
                div {
                    class: "flex gap-2",
                    a {
                        href: "https://github.com/nzuzo-newsnet",
                        class: "flex-1 py-2 text-center text-sm bg-primary text-primary-content rounded-md font-medium hover:bg-primary-focus transition-colors",
                        "Github"
                    }
                    a {
                        href: "mailto:nzuzo@newsnet.africa",
                        class: "flex-1 py-2 text-center text-sm rounded-md font-medium border border-base-300 hover:bg-base-200 transition-colors",
                        "Email"
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectItem(repo: GitHubRepo) -> Element {
    rsx! {
        a {
            href: "{repo.html_url}",
            target: "_blank",
            class: "flex items-start gap-3 p-2 rounded-lg transition-colors group hover:bg-base-200",

            // Icon Placeholder
            div {
                class: "mt-1 p-1.5 rounded-md bg-primary/10 text-primary",
                svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", xmlns: "http://www.w3.org/2000/svg",
                    path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" }
                }
            }

            div {
                class: "flex-1 min-w-0",
                div {
                    class: "flex items-center justify-between",
                    h4 { class: "font-semibold text-sm group-hover:text-primary transition-colors truncate", "{repo.name}" }
                    span { class: "text-xs flex items-center gap-0.5 opacity-50", "★ {repo.stargazers_count}" }
                }
                if let Some(desc) = &repo.description {
                    p { class: "text-xs truncate text-base-content/50", "{desc}" }
                }
                div {
                    class: "mt-1 flex gap-2",
                    if let Some(lang) = &repo.language {
                        span { class: "text-[10px] px-1.5 py-0.5 rounded border border-base-300 text-base-content/50", "{lang}" }
                    }
                }
            }
        }
    }
}
