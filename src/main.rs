use crate::shared::nav_bar::NavBar;
use dioxus::{
    document::eval,
    logger::{self, tracing::Level},
    prelude::*,
};

pub mod markdown_management;
pub mod pages;
pub mod shared;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },
    #[route("/about")]
    About {},
    #[route("/demos")]
    Demos {},
    #[route("/reading")]
    Reading {},
    #[route("/series")]
    Series {},
    #[route("/series/:name")]
    SeriesDetail { name: String },
    #[route("/articles")]
    Articles {},
    #[route("/demos/algovis")]
    AlgoVis {},
    #[route("/demos/neural-net")]
    NeuralNet {},
}

fn main() {
    // Initialize article watcher on server startup
    #[cfg(feature = "server")]
    {
        if let Err(e) = markdown_management::start_article_watcher() {
            logger::tracing::error!("Failed to start article watcher: {}", e);
        }
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _ = logger::init(Level::INFO);

    // Resolve the theme before first paint. Legacy daisyUI theme names from
    // earlier versions of the site are folded into the two custom themes so a
    // returning visitor isn't left on a palette that no longer exists.
    use_effect(move || {
        eval(
            r#"
            (function () {
              var DARK_LEGACY = ['dark','night','dracula','black','luxury','synthwave',
                'halloween','forest','aqua','business','coffee','dim','sunset'];
              var saved = null;
              try { saved = localStorage.getItem('theme'); } catch (e) {}

              var theme;
              if (saved === 'paper' || saved === 'ink') {
                theme = saved;
              } else if (saved) {
                theme = DARK_LEGACY.indexOf(saved) !== -1 ? 'ink' : 'paper';
              } else {
                theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'ink' : 'paper';
              }

              document.documentElement.setAttribute('data-theme', theme);
              try { localStorage.setItem('theme', theme); } catch (e) {}
            })();
            "#,
        );
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: "/favicon.svg", r#type: "image/svg+xml" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=5.0",
        }
        document::Meta {
            name: "description",
            content: "Nzuzo Magagula — engineer working on database internals, consensus algorithms and distributed systems in Rust.",
        }
        document::Meta { name: "theme-color", content: "#0f1420" }

        // The ambient field: three slow-drifting colour wells plus film grain,
        // replacing the blurred photograph the site used to sit on.
        div { class: "aurora", aria_hidden: "true",
            div { class: "aurora__well aurora__well--a" }
            div { class: "aurora__well aurora__well--b" }
            div { class: "aurora__well aurora__well--c" }
        }
        div { class: "grain", aria_hidden: "true" }

        Router::<Route> {}

        document::Script { src: "/js/motion.js", defer: true }
    }
}

/// Shared page shell: a viewport-height frame with the nav pinned at the top
/// and the child owning the scrolling region beneath it. `page-enter` gives
/// every navigation a short fade-and-rise.
#[component]
fn Shell(children: Element) -> Element {
    rsx! {
        div { class: "h-dvh flex flex-col overflow-hidden page-enter",
            NavBar {}
            {children}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Shell { pages::home_page::HomePage {} }
    }
}

#[component]
fn Article(segments: Vec<String>) -> Element {
    // Join segments to form the path (e.g., ["data-engineering", "01-pipeline-basics"] -> "data-engineering/01-pipeline-basics")
    let path = segments.join("/");
    let full_path = format!("{}.md", path);

    rsx! {
        Shell {
            // Use key to force component remount on path change
            // This ensures all hooks re-initialize with the new path
            pages::article_page::ArticlePage { key: "{full_path}", path: full_path.clone() }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        Shell { pages::about_page::AboutPage {} }
    }
}

#[component]
fn Demos() -> Element {
    rsx! {
        Shell { pages::demos_page::DemosPage {} }
    }
}

#[component]
fn Reading() -> Element {
    rsx! {
        Shell { pages::reading_page::ReadingPage {} }
    }
}

#[component]
fn Series() -> Element {
    rsx! {
        Shell { pages::series_page::SeriesPage {} }
    }
}

#[component]
fn SeriesDetail(name: String) -> Element {
    rsx! {
        Shell { pages::series_detail_page::SeriesDetailPage { series_name: name } }
    }
}

#[component]
fn Articles() -> Element {
    rsx! {
        Shell { pages::articles_page::ArticlesPage {} }
    }
}

#[component]
fn AlgoVis() -> Element {
    rsx! {
        Shell { pages::algo_vis_page::AlgoVisPage {} }
    }
}

#[component]
fn NeuralNet() -> Element {
    rsx! {
        Shell { pages::neural_net_page::NeuralNetPage {} }
    }
}
