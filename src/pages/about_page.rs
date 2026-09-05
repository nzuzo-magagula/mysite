use dioxus::prelude::*;
use dioxus_markdown::Markdown;

const ABOUT_ME_CONTENT: &str = include_str!("../../aboutme.md");

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        main { class: "flex-1 overflow-y-auto", "data-scroll-root": "true",
            div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-14 md:py-20",

                // ---- header ----
                header { class: "mb-12 flex flex-col sm:flex-row items-center sm:items-end gap-7", "data-reveal": "true",
                    div { class: "relative w-28 h-28 flex-shrink-0",
                        div {
                            class: "absolute -inset-1.5 rounded-[1.4rem] opacity-60 animate-spin-slow",
                            style: "background: conic-gradient(from 0deg, var(--color-primary), var(--color-secondary), var(--color-accent), var(--color-primary)); filter: blur(14px);",
                            aria_hidden: "true",
                        }
                        img {
                            src: "/main_image.jpg",
                            alt: "Nzuzo Magagula",
                            class: "relative w-full h-full object-cover rounded-[1.2rem] border border-[var(--hairline-strong)] shadow-[var(--elev-2)]",
                        }
                    }

                    div { class: "text-center sm:text-left",
                        p { class: "eyebrow mb-2", "About" }
                        h1 { class: "text-4xl md:text-5xl font-semibold text-gradient animate-gradient-pan",
                            "Nzuzo Magagula"
                        }
                        p { class: "mt-2 text-base-content/60",
                            "Engineer — Rust, databases, distributed systems."
                        }
                    }
                }

                // ---- body ----
                article {
                    class: "surface p-7 md:p-11",
                    "data-reveal": "true",
                    style: "--reveal-delay: 100ms",
                    div { class: "prose prose-lg max-w-none",
                        Markdown { content: ABOUT_ME_CONTENT.to_string() }
                    }
                }

                // ---- contact strip ----
                div {
                    class: "grid sm:grid-cols-2 gap-4 mt-6",
                    "data-reveal": "true",
                    style: "--reveal-delay: 180ms",

                    ContactCard {
                        label: "General",
                        address: "me@nzuzomagagula.online",
                        note: "Anything at all",
                    }
                    ContactCard {
                        label: "Development",
                        address: "dev@nzuzomagagula.online",
                        note: "Code, collaboration, consulting",
                    }
                }
            }
        }
    }
}

#[component]
fn ContactCard(label: String, address: String, note: String) -> Element {
    rsx! {
        a {
            href: "mailto:{address}",
            class: "surface card-interactive card-spotlight group p-5 block",

            p { class: "eyebrow mb-2 relative z-[2]", "{label}" }
            p { class: "font-mono text-sm font-semibold text-primary break-all relative z-[2]",
                "{address}"
            }
            p { class: "text-xs text-base-content/50 mt-2 relative z-[2]", "{note}" }
        }
    }
}
