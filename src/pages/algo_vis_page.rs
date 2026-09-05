use crate::pages::neural_net_page::DemoChrome;
use dioxus::prelude::*;

#[component]
pub fn AlgoVisPage() -> Element {
    rsx! {
        main { class: "flex-1 flex flex-col overflow-hidden",
            DemoChrome {
                title: "Algorithm Visualiser",
                subtitle: "React 18 · sorting and pathfinding, stepped one operation at a time",
            }
            div { class: "flex-1 min-h-0",
                iframe {
                    src: "/algovis/index.html",
                    class: "w-full h-full border-none bg-transparent animate-fade",
                    title: "Algorithm Visualizer Demo",
                    allowfullscreen: true,
                }
            }
        }
    }
}
