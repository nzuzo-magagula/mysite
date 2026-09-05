use crate::Route;
use dioxus::prelude::*;

/// Hosts the Svelte + Pyodide demo. The iframe keeps the demo's own toolchain
/// (Svelte 5, Vite) fully separate from the Dioxus app, exactly as the
/// algorithm visualiser does — the two only agree on a theme message.
#[component]
pub fn NeuralNetPage() -> Element {
    rsx! {
        main { class: "flex-1 flex flex-col overflow-hidden",
            DemoChrome {
                title: "Tiny Neural Network",
                subtitle: "Svelte 5 · Python 3 on Pyodide · trained entirely client-side",
            }
            div { class: "flex-1 min-h-0",
                iframe {
                    src: "/neuralnet/index.html",
                    class: "w-full h-full border-none bg-transparent animate-fade",
                    title: "Tiny neural network training visualiser",
                    "loading": "lazy",
                }
            }
        }
    }
}

/// Slim breadcrumb bar shared by the embedded demos.
#[component]
pub fn DemoChrome(title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "flex-shrink-0 border-b border-[var(--hairline)] bg-base-100/45 backdrop-blur-xl",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-2.5 flex items-center gap-4",
                Link {
                    to: Route::Demos {},
                    class: "group inline-flex items-center gap-1.5 text-sm font-medium text-base-content/60 hover:text-primary transition-colors duration-300",
                    svg {
                        class: "w-4 h-4 transition-transform duration-300 group-hover:-translate-x-1",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M15 19l-7-7 7-7" }
                    }
                    "Demos"
                }
                div { class: "h-4 w-px bg-[var(--hairline-strong)]" }
                div { class: "min-w-0",
                    p { class: "font-display font-semibold text-sm leading-tight truncate", "{title}" }
                    p { class: "eyebrow text-[0.58rem] truncate", "{subtitle}" }
                }
            }
        }
    }
}
