pub mod blog;
pub mod projects;
use crate::pages::home_page::{blog::Blogs, projects::Projects};
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        main { class: "flex-1 w-full overflow-y-auto", "data-scroll-root": "true",

            Hero {}

            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-16",
                div { class: "flex flex-col lg:flex-row gap-10",
                    div { class: "flex-1 min-w-0", Blogs {} }
                    aside { class: "lg:w-[19rem] flex-shrink-0",
                        div { class: "lg:sticky lg:top-6", Projects {} }
                    }
                }
            }

            SiteFooter {}
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "relative",
            // Faint engineering grid, masked to a soft ellipse behind the text.
            div { class: "absolute inset-0 grid-field pointer-events-none", aria_hidden: "true" }

            div { class: "relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 md:py-24",
                div { class: "flex flex-col-reverse md:flex-row items-center justify-between gap-10 md:gap-14",

                    // ---- copy ----
                    div { class: "flex-1 max-w-2xl text-center md:text-left",
                        p { class: "eyebrow mb-4", "data-reveal": "true", "Nzuzo Magagula — Engineer" }

                        h1 {
                            class: "text-[2.5rem] leading-[1.05] md:text-6xl md:leading-[1.02] font-semibold mb-6",
                            "data-reveal": "true",
                            style: "--reveal-delay: 80ms",
                            span { class: "text-gradient animate-gradient-pan",
                                "Building high-performance distributed systems in Rust."
                            }
                        }

                        p {
                            class: "text-lg md:text-xl mb-8 text-base-content/70 leading-relaxed",
                            "data-reveal": "true",
                            style: "--reveal-delay: 160ms",
                            "Database internals, consensus algorithms, and the parts of internet
                             infrastructure most people never have to think about."
                        }

                        div {
                            class: "flex flex-wrap gap-3 justify-center md:justify-start",
                            "data-reveal": "true",
                            style: "--reveal-delay: 240ms",

                            Link { to: "/articles", class: "btn-solid", "Read the writing" }

                            Link { to: "/demos", class: "btn-ghost-outline",
                                "Try a demo"
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 5l7 7-7 7" }
                                }
                            }
                        }

                        // A few grounding facts rather than another row of buttons.
                        dl {
                            class: "mt-12 grid grid-cols-3 gap-6 max-w-md mx-auto md:mx-0",
                            "data-reveal": "true",
                            style: "--reveal-delay: 320ms",
                            Stat { value: "Rust", label: "primary language" }
                            Stat { value: "WASM", label: "this site's runtime" }
                            Stat { value: "OSS", label: "everything public" }
                        }
                    }

                    // ---- portrait ----
                    Link {
                        to: "/about",
                        class: "group relative flex-shrink-0 w-52 h-52 md:w-[19rem] md:h-[19rem]",
                        "data-reveal": "scale",
                        style: "--reveal-delay: 120ms",

                        // Rotating conic ring behind the portrait.
                        div {
                            class: "absolute -inset-3 rounded-[2rem] opacity-0 group-hover:opacity-100 transition-opacity duration-700 animate-spin-slow",
                            style: "background: conic-gradient(from 0deg, var(--color-primary), var(--color-secondary), var(--color-accent), var(--color-primary)); filter: blur(26px);",
                            aria_hidden: "true",
                        }

                        div {
                            class: "relative w-full h-full rounded-[1.75rem] overflow-hidden border border-[var(--hairline-strong)] shadow-[var(--elev-3)] transition-transform duration-700 ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-[1.03] group-hover:-rotate-1",
                            img {
                                src: "/main_image.jpg",
                                alt: "Nzuzo Magagula",
                                class: "w-full h-full object-cover transition-transform duration-[1200ms] ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-110",
                            }
                            // Caption slides up on hover.
                            div { class: "absolute inset-x-0 bottom-0 p-4 translate-y-full group-hover:translate-y-0 transition-transform duration-500 ease-[cubic-bezier(.22,1,.36,1)] bg-gradient-to-t from-black/75 to-transparent",
                                span { class: "text-white text-sm font-semibold", "About me →" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Stat(value: String, label: String) -> Element {
    rsx! {
        div {
            dt { class: "font-display text-2xl font-semibold text-primary", "{value}" }
            dd { class: "eyebrow text-[0.6rem] mt-1", "{label}" }
        }
    }
}

#[component]
fn SiteFooter() -> Element {
    rsx! {
        footer { class: "surface-flush border-x-0 border-b-0 mt-8",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10",
                div { class: "flex flex-col md:flex-row justify-between items-center gap-6",
                    div { class: "text-center md:text-left",
                        p { class: "font-display font-semibold", "Nzuzo Magagula" }
                        p { class: "text-sm text-base-content/55 mt-1",
                            "Built with Rust, Dioxus and WebAssembly."
                        }
                    }

                    nav { class: "flex flex-wrap justify-center gap-x-7 gap-y-2 text-sm font-medium",
                        a {
                            href: "https://github.com/nzuzo-newsnet",
                            class: "link-underline text-base-content/65 hover:text-primary transition-colors duration-300",
                            "GitHub"
                        }
                        a {
                            href: "mailto:me@nzuzomagagula.online",
                            class: "link-underline text-base-content/65 hover:text-primary transition-colors duration-300",
                            "Email"
                        }
                        Link {
                            to: "/reading",
                            class: "link-underline text-base-content/65 hover:text-primary transition-colors duration-300",
                            "Reading"
                        }
                        Link {
                            to: "/about",
                            class: "link-underline text-base-content/65 hover:text-primary transition-colors duration-300",
                            "About"
                        }
                    }
                }

                p { class: "eyebrow text-[0.6rem] mt-8 text-center md:text-left",
                    "© 2026 Nzuzo Magagula"
                }
            }
        }
    }
}
