use dioxus::document::eval;
use dioxus::prelude::*;

/// The two hand-built themes. Everything else the site used to ship (33 stock
/// daisyUI palettes) is gone — these are defined by hand in `tailwind.css`.
const THEME_LIGHT: &str = "paper";
const THEME_DARK: &str = "ink";

#[component]
pub fn NavBar() -> Element {
    let mut theme = use_signal(|| THEME_LIGHT.to_string());
    let mut is_loaded = use_signal(|| false);
    let mut is_mobile_menu_open = use_signal(|| false);
    let current_route = use_route::<crate::Route>();

    // Read back whatever App's boot script settled on, so the toggle icon
    // matches the palette already painted.
    use_effect(move || {
        if !is_loaded() {
            spawn(async move {
                let read_script = r#"
                    try {
                        dioxus.send(document.documentElement.getAttribute('data-theme') || 'paper');
                    } catch (e) {
                        dioxus.send('paper');
                    }
                "#;

                if let Ok(active) = eval(read_script).recv::<String>().await {
                    theme.set(active);
                    is_loaded.set(true);
                }
            });
        }
    });

    // Persist and broadcast theme changes.
    use_effect(move || {
        if is_loaded() {
            let current = theme.read().clone();
            let _ = eval(&format!(
                r#"
                (function () {{
                  var t = {theme};
                  document.documentElement.setAttribute('data-theme', t);
                  try {{ localStorage.setItem('theme', t); }} catch (e) {{}}

                  // Demos run in iframes and mirror the parent palette.
                  document.querySelectorAll('iframe').forEach(function (frame) {{
                    try {{
                      frame.contentWindow.postMessage({{ type: 'THEME_CHANGE', theme: t }}, '*');
                    }} catch (e) {{}}
                  }});
                }})();
                "#,
                theme = serde_json::to_string(&current).unwrap_or_else(|_| "\"paper\"".into())
            ));
        }
    });

    let is_dark = theme.read().as_str() == THEME_DARK;

    let nav_links = [
        ("/", "Home"),
        ("/articles", "Articles"),
        ("/series", "Series"),
        ("/demos", "Demos"),
        ("/reading", "Reading"),
        ("/about", "About"),
    ];

    let is_active = |path: &str| -> bool {
        match (&current_route, path) {
            (crate::Route::Home {}, "/") => true,
            (crate::Route::Articles {}, "/articles") => true,
            (crate::Route::Series {}, "/series") => true,
            (crate::Route::SeriesDetail { .. }, "/series") => true,
            (crate::Route::Demos {}, "/demos") => true,
            (crate::Route::AlgoVis {}, "/demos") => true,
            (crate::Route::NeuralNet {}, "/demos") => true,
            (crate::Route::Reading {}, "/reading") => true,
            (crate::Route::About {}, "/about") => true,
            _ => false,
        }
    };

    rsx! {
        nav {
            "data-nav": "true",
            class: "relative z-50 flex-shrink-0 surface-flush border-x-0 border-t-0 transition-shadow duration-500",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center h-16",

                    // ---- Wordmark ----
                    Link {
                        to: "/",
                        class: "flex-shrink-0 flex items-center gap-3 group",
                        div {
                            class: "relative w-9 h-9 rounded-[0.7rem] overflow-hidden transition-transform duration-500 ease-[cubic-bezier(.34,1.56,.64,1)] group-hover:scale-110 group-hover:-rotate-6",
                            img {
                                src: "/favicon.svg",
                                alt: "",
                                class: "w-full h-full",
                            }
                        }
                        div { class: "hidden sm:flex flex-col leading-none",
                            span { class: "font-display font-semibold text-[1.05rem] tracking-tight",
                                "Nzuzo Magagula"
                            }
                            span { class: "eyebrow text-[0.6rem] mt-0.5", "Systems / Rust" }
                        }
                    }

                    // ---- Desktop links ----
                    div { class: "hidden md:flex items-center gap-1",
                        for (path , label) in nav_links.iter() {
                            Link {
                                to: path.to_string(),
                                class: if is_active(path) {
                                    "relative px-3 py-2 text-sm font-semibold text-primary transition-colors duration-300"
                                } else {
                                    "relative px-3 py-2 text-sm font-medium text-base-content/65 hover:text-base-content transition-colors duration-300"
                                },
                                "{label}"
                                if is_active(path) {
                                    span { class: "absolute left-3 right-3 -bottom-px h-[2px] rounded-full bg-primary animate-fade" }
                                }
                            }
                        }

                        span { class: "mx-2 h-5 w-px bg-[var(--hairline-strong)]" }

                        ThemeToggle { is_dark, on_toggle: move |_| {
                            let next = if is_dark { THEME_LIGHT } else { THEME_DARK };
                            theme.set(next.to_string());
                        } }
                    }

                    // ---- Mobile controls ----
                    div { class: "md:hidden flex items-center gap-1",
                        ThemeToggle { is_dark, on_toggle: move |_| {
                            let next = if is_dark { THEME_LIGHT } else { THEME_DARK };
                            theme.set(next.to_string());
                        } }
                        button {
                            class: "p-2 rounded-[var(--radius-field)] hover:bg-base-200/70 transition-colors duration-300",
                            aria_label: "Toggle navigation",
                            aria_expanded: if is_mobile_menu_open() { "true" } else { "false" },
                            onclick: move |_| is_mobile_menu_open.toggle(),
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                if is_mobile_menu_open() {
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M6 18L18 6M6 6l12 12" }
                                } else {
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 7h16M4 12h16M4 17h16" }
                                }
                            }
                        }
                    }
                }
            }

            // Reading progress, filled by motion.js on pages that scroll.
            div { class: "read-progress", "data-read-progress": "true", aria_hidden: "true" }

            // ---- Mobile drawer ----
            if is_mobile_menu_open() {
                div { class: "md:hidden border-t border-[var(--hairline)] bg-base-100/95 backdrop-blur-xl animate-fade",
                    div { class: "px-3 py-3 space-y-1",
                        for (i , (path , label)) in nav_links.iter().enumerate() {
                            Link {
                                to: path.to_string(),
                                "data-reveal": "left",
                                style: "--reveal-delay: {i * 45}ms",
                                class: if is_active(path) {
                                    "block px-3 py-2.5 rounded-[var(--radius-field)] text-base font-semibold text-primary bg-primary/10"
                                } else {
                                    "block px-3 py-2.5 rounded-[var(--radius-field)] text-base font-medium text-base-content/70 hover:bg-base-200/70 hover:text-base-content transition-colors"
                                },
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Sun/moon toggle whose two icons cross-fade and counter-rotate.
#[component]
fn ThemeToggle(is_dark: bool, on_toggle: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "relative p-2 rounded-full hover:bg-base-200/70 transition-colors duration-300 group",
            aria_label: if is_dark { "Switch to the light theme" } else { "Switch to the dark theme" },
            onclick: move |_| on_toggle.call(()),

            div { class: "relative w-5 h-5",
                // Sun
                svg {
                    class: if is_dark {
                        "absolute inset-0 w-5 h-5 text-accent transition-all duration-500 ease-[cubic-bezier(.22,1,.36,1)] opacity-100 rotate-0 scale-100"
                    } else {
                        "absolute inset-0 w-5 h-5 text-accent transition-all duration-500 ease-[cubic-bezier(.22,1,.36,1)] opacity-0 -rotate-90 scale-50"
                    },
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    circle { cx: "12", cy: "12", r: "4" }
                    path {
                        stroke_linecap: "round",
                        d: "M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32l1.41 1.41M2 12h2m16 0h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41",
                    }
                }
                // Moon
                svg {
                    class: if is_dark {
                        "absolute inset-0 w-5 h-5 text-base-content/70 transition-all duration-500 ease-[cubic-bezier(.22,1,.36,1)] opacity-0 rotate-90 scale-50"
                    } else {
                        "absolute inset-0 w-5 h-5 text-base-content/70 transition-all duration-500 ease-[cubic-bezier(.22,1,.36,1)] opacity-100 rotate-0 scale-100"
                    },
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z",
                    }
                }
            }
        }
    }
}
