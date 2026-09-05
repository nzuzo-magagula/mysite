use dioxus::{logger, prelude::*};
use dioxus_markdown::Markdown;

use crate::markdown_management::{ArticleTomlMetadata, fetch_article_with_metadata};

#[component]
pub fn ArticlePage(path: String) -> Element {
    let active_tab = use_signal(|| "article".to_string()); //TODO: Use enum

    logger::tracing::info!(
        "ArticlePage component mounted/remounted with path: {}",
        path
    );

    // Store path in a signal to make it explicitly reactive
    let mut current_path = use_signal(|| path.clone());

    // Update the signal if the path prop changes (redundant when using key, but safe)
    use_effect(move || {
        logger::tracing::info!("Path effect triggered, updating to: {}", path);
        current_path.set(path.clone());
    });

    // Fetch article with metadata from server
    // Resource will restart when current_path signal changes
    let article_data = use_resource(move || {
        let path_to_fetch = current_path();

        async move {
            logger::tracing::info!("Fetching article: {}", path_to_fetch);
            let result = fetch_article_with_metadata(path_to_fetch.clone()).await;
            match &result {
                Ok(_) => {
                    logger::tracing::info!("Successfully fetched article: {}", path_to_fetch)
                }
                Err(e) => {
                    logger::tracing::error!("Failed to fetch article {}: {:?}", path_to_fetch, e)
                }
            }
            result
        }
    });

    rsx! {
        main {
            class: "flex-1 overflow-hidden flex flex-col md:flex-row",

            // Main content area
            div {
                class: "flex-1 overflow-y-auto px-4 md:px-8 py-8 pb-32 md:pb-12",
                "data-scroll-root": "true",
                div {
                    class: "mx-auto max-w-4xl",
                article {
                    class: "surface p-6 md:p-10",
                    section {
                        class: "",
                    {
                        match article_data.read().as_ref() {
                            Some(Ok(article)) => {
                                logger::tracing::info!("Rendering article: {}", article.metadata.path);
                                rsx! {
                                    div {
                                        class: "space-y-6",

                                        // Article metadata
                                        if let Some(ref meta) = article.toml_metadata {
                                            ArticleMetadata {
                                                metadata: meta.clone()
                                            }
                                        }

                                        // Tab content
                                        div {
                                            class: "min-h-[500px]",
                                            match active_tab.read().as_str() {
                                                "article" => {
                                                dioxus::logger::tracing::info!("Rendering Article: {:?}", article.metadata.path);
                                                    rsx! {
                                                    div {
                                                        class: "prose prose-lg max-w-none",
                                                        Markdown {
                                                            content: article.content.clone(),
                                                        }
                                                    }

                                                    // Next/Previous Navigation Cards
                                                    if let Some(ref meta) = article.toml_metadata {
                                                        NavigationCards {
                                                            metadata: meta.clone()
                                                        }
                                                    }

                                                    // Tags section
                                                    if let Some(ref meta) = article.toml_metadata {
                                                        if !meta.tags.is_empty() {
                                                            div {
                                                                class: "border-t border-[var(--hairline)] pt-6 mt-8",
                                                                h3 {
                                                                    class: "text-lg font-semibold mb-3",
                                                                    "Tags"
                                                                }
                                                                div {
                                                                    class: "flex flex-wrap gap-2",
                                                                    for tag in &meta.tags {
                                                                        span {
                                                                            class: "chip h-fit",
                                                                            "#{tag}"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }},
                                                "references" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-6",
                                                            "References & Resources"
                                                        }

                                                        // Display references if available in metadata
                                                        if let Some(ref meta) = article.toml_metadata {
                                                            if !meta.references.is_empty() {
                                                                div {
                                                                    class: "space-y-4",
                                                                    for (index, reference) in meta.references.iter().enumerate() {
                                                                        div {
                                                                            class: "surface-sunken p-4 transition-colors duration-300 hover:border-primary/35",
                                                                            div {
                                                                                class: "card-body",
                                                                                div {
                                                                                    class: "flex items-start gap-3",

                                                                                    // Reference number badge
                                                                                    span {
                                                                                        class: "flex items-center justify-center w-7 h-7 rounded-full font-mono text-xs font-bold bg-primary/12 text-primary border border-primary/25 shrink-0 mt-0.5",
                                                                                        "{index + 1}"
                                                                                    }

                                                                                    // Reference content
                                                                                    div {
                                                                                        class: "flex-1 min-w-0",

                                                                                        // Title with link
                                                                                        a {
                                                                                            href: "{reference.url}",
                                                                                            target: "_blank",
                                                                                            rel: "noopener noreferrer",
                                                                                            class: "font-semibold text-primary hover:underline break-words",
                                                                                            "{reference.title}"
                                                                                        }

                                                                                        // URL display
                                                                                        div {
                                                                                            class: "text-sm opacity-70 mt-1 break-all",
                                                                                            "{reference.url}"
                                                                                        }

                                                                                        // Description if available
                                                                                        if let Some(ref description) = reference.description {
                                                                                            p {
                                                                                                class: "text-sm mt-2 opacity-80",
                                                                                                "{description}"
                                                                                            }
                                                                                        }
                                                                                    }

                                                                                    // External link icon
                                                                                    svg {
                                                                                        xmlns: "http://www.w3.org/2000/svg",
                                                                                        class: "h-5 w-5 shrink-0 opacity-50 mt-1",
                                                                                        fill: "none",
                                                                                        view_box: "0 0 24 24",
                                                                                        stroke: "currentColor",
                                                                                        path {
                                                                                            stroke_linecap: "round",
                                                                                            stroke_linejoin: "round",
                                                                                            stroke_width: "2",
                                                                                            d: "M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                p {
                                                                    class: "text-base-content opacity-70",
                                                                    "No references have been added to this article yet."
                                                                }
                                                            }
                                                        } else {
                                                            p {
                                                                class: "text-base-content opacity-70",
                                                                "No references have been added to this article yet."
                                                            }
                                                        }
                                                    }
                                                },
                                                "demo" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Interactive Demo"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Interactive demos will appear here once configured in article metadata."
                                                        }
                                                    }
                                                },
                                                "related" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Related Articles"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Related articles will be suggested here based on topics and series."
                                                        }
                                                    }
                                                },
                                                "quiz" => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        h2 {
                                                            class: "text-xl font-bold mb-4",
                                                            "Check Your Knowledge"
                                                        }
                                                        p {
                                                            class: "text-base-content opacity-70",
                                                            "Knowledge checks and quizzes will appear here."
                                                        }
                                                    }
                                                },
                                                _ => rsx! {
                                                    div {
                                                        class: "p-8",
                                                        "Content not available"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                div {
                                    class: "surface p-10 text-center border-error/30",
                                    h3 { class: "text-xl font-semibold mb-2 text-error", "Could not load this article" }
                                    p { class: "text-sm font-mono text-base-content/55 break-all", "{e}" }
                                }
                            },
                            None => rsx! {
                                ArticleSkeleton {}
                            }
                        }
                    }
                }
            }
                }
            }

            // Right sidebar for md+ screens with tabs
            {
                if let Some(Ok(article)) = article_data.read().as_ref() {
                    rsx! {
                        div {
                            class: "hidden md:block md:w-80 lg:w-96 border-l border-[var(--hairline)] bg-base-100/45 backdrop-blur-xl overflow-y-auto",
                            RightSidebar {
                                active_tab: active_tab,
                                metadata: article.toml_metadata.clone()
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }

            // Bottom Navigation Panel for mobile only
            {
                if let Some(Ok(article)) = article_data.read().as_ref() {
                    rsx! {
                        div {
                            class: "md:hidden",
                            BottomNavPanel {
                                active_tab: active_tab,
                                metadata: article.toml_metadata.clone()
                            }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

#[component]
fn RightSidebar(active_tab: Signal<String>, metadata: Option<ArticleTomlMetadata>) -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full",

            // Navigation section at top
            if let Some(ref meta) = metadata {
                div {
                    class: "p-4 border-b border-[var(--hairline)]",
                    h3 {
                        class: "text-sm font-semibold mb-3 opacity-70",
                        "Navigate"
                    }

                    // Series info
                    if !meta.article_series.is_empty() {
                        {
                            let series = &meta.article_series[0];
                            rsx! {
                                div {
                                    class: "mb-3",
                                    span {
                                        class: "chip",
                                        "{series.name}"
                                    }
                                }
                            }
                        }
                    }

                    // Prev/Next buttons
                    div {
                        class: "flex gap-2",
                        if !meta.article_series.is_empty() {
                            {
                                let series = &meta.article_series[0];
                                rsx! {
                                    if let Some(ref prev) = series.prev {
                                        NavigationButton {
                                            to: format!("/article/{}", prev),
                                            class: "pill pill--off flex-1 text-center",
                                            label: "← Prev"
                                        }
                                    }
                                    if let Some(ref next) = series.next {
                                        NavigationButton {
                                            to: format!("/article/{}", next),
                                            class: "pill pill--on flex-1 text-center",
                                            label: "Next →"
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(ref prev) = meta.prev_article {
                                NavigationButton {
                                    to: format!("/article/{}", prev),
                                    class: "pill pill--off flex-1 text-center",
                                    label: "← Prev"
                                }
                            }
                            if let Some(ref next) = meta.next_article {
                                NavigationButton {
                                    to: format!("/article/{}", next),
                                    class: "pill pill--on flex-1 text-center",
                                    label: "Next →"
                                }
                            }
                        }
                    }
                }
            }

            // Tabs section
            div {
                class: "flex-1 p-4",
                h3 {
                    class: "text-sm font-semibold mb-3 opacity-70",
                    "Sections"
                }

                div {
                    class: "flex flex-col gap-2",

                    // Article tab
                    button {
                        class: if active_tab.read().as_str() == "article" {
                            "pill pill--on w-full text-left justify-start"
                        } else {
                            "pill pill--off w-full text-left justify-start"
                        },
                        onclick: move |_| active_tab.set("article".to_string()),
                        "📄 Article"
                    }

                    // References tab
                    if metadata.as_ref().map(|m| m.show_references).unwrap_or(true) {
                        button {
                            class: if active_tab.read().as_str() == "references" {
                                "pill pill--on w-full text-left justify-start"
                            } else {
                                "pill pill--off w-full text-left justify-start"
                            },
                            onclick: move |_| active_tab.set("references".to_string()),
                            "📚 References"
                        }
                    }

                    // Demo tab
                    if metadata.as_ref().map(|m| m.show_demo).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "demo" {
                                "pill pill--on w-full text-left justify-start"
                            } else {
                                "pill pill--off w-full text-left justify-start"
                            },
                            onclick: move |_| active_tab.set("demo".to_string()),
                            "▶️ Demo"
                        }
                    }

                    // Related tab
                    if metadata.as_ref().map(|m| m.show_related).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "related" {
                                "pill pill--on w-full text-left justify-start"
                            } else {
                                "pill pill--off w-full text-left justify-start"
                            },
                            onclick: move |_| active_tab.set("related".to_string()),
                            "⚡ Related"
                        }
                    }

                    // Quiz tab
                    if metadata.as_ref().map(|m| m.show_quiz).unwrap_or(false) {
                        button {
                            class: if active_tab.read().as_str() == "quiz" {
                                "pill pill--on w-full text-left justify-start"
                            } else {
                                "pill pill--off w-full text-left justify-start"
                            },
                            onclick: move |_| active_tab.set("quiz".to_string()),
                            "✓ Quiz"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleMetadata(metadata: ArticleTomlMetadata) -> Element {
    rsx! {
        div {
            class: "surface-sunken p-6 space-y-4",

            // Author and date
            div {
                class: "flex flex-wrap items-center gap-4 text-sm",
                if let Some(ref author) = metadata.author {
                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "font-semibold",
                            "By {author}"
                        }
                    }
                }
                if let Some(ref date) = metadata.date {
                    div {
                        class: "flex items-center gap-2",
                        span { class: "chip", "{date}" }
                    }
                }
                if let Some(ref reading_time) = metadata.reading_time {
                    div {
                        class: "flex items-center gap-2",
                        span { class: "chip", "{reading_time}" }
                    }
                }
            }

            // Summary
            if let Some(ref summary) = metadata.summary {
                div {
                    class: "text-base-content/70 border-l-2 border-primary pl-4 leading-relaxed",
                    p { "{summary}" }
                }
            }

            // Topics and category
            div {
                class: "flex flex-wrap gap-2",
                if let Some(ref category) = metadata.category {
                    span {
                        class: "chip",
                        style: "color: var(--color-primary); border-color: color-mix(in oklab, var(--color-primary) 35%, transparent);",
                        "{category}"
                    }
                }
                for topic in &metadata.topics {
                    span { class: "chip", "{topic}" }
                }
            }

            // Thumbnail
            if let Some(ref thumbnail) = metadata.thumbnail {
                div {
                    class: "mt-4",
                    img {
                        src: "{thumbnail}",
                        alt: "Article thumbnail",
                        class: "rounded-[var(--radius-box)] border border-[var(--hairline)] w-full max-h-96 object-cover"
                    }
                }
            }
        }
    }
}

#[component]
fn ArticleSkeleton() -> Element {
    rsx! {
        div {
            class: "space-y-6 p-2",

            // Metadata skeleton
            div {
                class: "bg-gray-200 rounded-lg p-6 space-y-4",
                div {
                    class: "flex gap-4",
                    div { class: "h-4 bg-gray-300 rounded w-32" }
                    div { class: "h-4 bg-gray-300 rounded w-24" }
                }
                div { class: "h-16 bg-gray-300 rounded" }
                div {
                    class: "flex gap-2",
                    div { class: "h-6 bg-gray-300 rounded w-20" }
                    div { class: "h-6 bg-gray-300 rounded w-24" }
                    div { class: "h-6 bg-gray-300 rounded w-20" }
                }
            }

            // Title skeleton
            div { class: "h-10 bg-gray-200 rounded w-3/4" }

            // Content skeleton
            div {
                class: "space-y-3",
                div { class: "h-4 bg-gray-200 rounded" }
                div { class: "h-4 bg-gray-200 rounded w-5/6" }
                div { class: "h-4 bg-gray-200 rounded w-4/6" }
            }
            div {
                class: "space-y-3 pt-4",
                div { class: "h-4 bg-gray-200 rounded" }
                div { class: "h-4 bg-gray-200 rounded w-5/6" }
            }
        }
    }
}

#[component]
fn BottomNavPanel(active_tab: Signal<String>, metadata: Option<ArticleTomlMetadata>) -> Element {
    rsx! {
        div {
            class: "fixed bottom-0 left-0 right-0 z-50 surface-flush border-x-0 border-b-0 shadow-[var(--elev-3)]",

            // Article Tab (Required)
            button {
                class: if active_tab.read().as_str() == "article" { "dock-active" } else { "" },
                onclick: move |_| active_tab.set("article".to_string()),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    }
                }
                span {
                    class: "dock-label",
                    "Article"
                }
            }

            // Next/Prev Navigation (Required)
            div {
                class: "flex flex-col items-center justify-center cursor-default gap-1",

                // Series name badge if using new structure
                if let Some(ref meta) = metadata {
                    if !meta.article_series.is_empty() {
                        span {
                            class: "chip",
                            "{meta.article_series[0].name}"
                        }
                    }
                }

                div {
                    class: "flex gap-2",
                    if let Some(ref meta) = metadata {
                        // Use new article_series structure
                        if !meta.article_series.is_empty() {
                            {
                                let series = &meta.article_series[0];
                                rsx! {
                                    if let Some(ref prev) = series.prev {
                                        NavigationButton {
                                            to: format!("/article/{}", prev),
                                            class: "pill pill--off !px-2.5 !py-1 !text-xs",
                                            label: "← Prev"
                                        }
                                    }
                                    if let Some(ref next) = series.next {
                                        NavigationButton {
                                            to: format!("/article/{}", next),
                                            class: "pill pill--off !px-2.5 !py-1 !text-xs",
                                            label: "Next →"
                                        }
                                    }
                                }
                            }
                        } else {
                            // Fall back to legacy fields
                            if let Some(ref prev) = meta.prev_article {
                                NavigationButton {
                                    to: format!("/article/{}", prev),
                                    class: "pill pill--off !px-2.5 !py-1 !text-xs",
                                    label: "← Prev"
                                }
                            }
                            if let Some(ref next) = meta.next_article {
                                NavigationButton {
                                    to: format!("/article/{}", next),
                                    class: "pill pill--off !px-2.5 !py-1 !text-xs",
                                    label: "Next →"
                                }
                            }
                        }
                    } else {
                        span {
                            class: "text-xs opacity-50",
                            "No navigation"
                        }
                    }
                }
            }

            // References Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_references).unwrap_or(true) {
                button {
                    class: if active_tab.read().as_str() == "references" { "dock-active" } else { "" },
                    onclick: move |_| active_tab.set("references".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                        }
                    }
                    span {
                        class: "dock-label",
                        "References"
                    }
                }
            }

            // Demo Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_demo).unwrap_or(false) {
                button {
                    class: if active_tab.read().as_str() == "demo" { "dock-active" } else { "" },
                    onclick: move |_| active_tab.set("demo".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                        }
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                    span {
                        class: "dock-label",
                        "Demo"
                    }
                }
            }

            // Related Articles Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_related).unwrap_or(false) {
                button {
                    class: if active_tab.read().as_str() == "related" { "dock-active" } else { "" },
                    onclick: move |_| active_tab.set("related".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M13 10V3L4 14h7v7l9-11h-7z"
                        }
                    }
                    span {
                        class: "dock-label",
                        "Related"
                    }
                }
            }

            // Quiz Tab (Conditional)
            if metadata.as_ref().map(|m| m.show_quiz).unwrap_or(false) {
                button {
                    class: if active_tab.read().as_str() == "quiz" { "dock-active" } else { "" },
                    onclick: move |_| active_tab.set("quiz".to_string()),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-5 w-5",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"
                        }
                    }
                    span {
                        class: "dock-label",
                        "Quiz"
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationCards(metadata: ArticleTomlMetadata) -> Element {
    // Determine prev and next from article_series or legacy fields
    let (prev_path, next_path, series_name) = if !metadata.article_series.is_empty() {
        let series = &metadata.article_series[0];
        (
            series.prev.clone(),
            series.next.clone(),
            Some(series.name.clone()),
        )
    } else {
        (
            metadata.prev_article.clone(),
            metadata.next_article.clone(),
            metadata.primary_series.clone(),
        )
    };

    // Only render if there's at least one navigation link
    if prev_path.is_none() && next_path.is_none() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "border-t border-[var(--hairline)] pt-8 mt-8",

            // Series name badge if available
            if let Some(ref series) = series_name {
                div {
                    class: "mb-4",
                    span {
                        class: "chip",
                        "{series}"
                    }
                }
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",

                // Previous article card
                if let Some(ref prev) = prev_path {
                    NavigationCard {
                        to: format!("/article/{}", prev),
                        direction: "Previous".to_string(),
                        title: prev.split('/').last().unwrap_or(prev).replace("-", " ")
                    }
                } else {
                    // Empty placeholder for grid alignment
                    div {}
                }

                // Next article card
                if let Some(ref next) = next_path {
                    NavigationCard {
                        to: format!("/article/{}", next),
                        direction: "next".to_string(),
                        title: next.split('/').last().unwrap_or(next).replace("-", " ")
                    }
                }
            }
        }
    }
}

/// Navigation button using full page redirect
#[component]
fn NavigationButton(to: String, class: String, label: String) -> Element {
    rsx! {
        a {
            href: "{to}",
            class: "{class}",
            "{label}"
        }
    }
}

/// Navigation card for prev/next article navigation
#[component]
fn NavigationCard(to: String, direction: String, title: String) -> Element {
    let is_next = direction == "next";

    rsx! {
        a {
            href: "{to}",
            class: "surface-sunken p-4 block transition-colors duration-300 hover:border-primary/35",
            div {
                class: "card-body",
                div {
                    class: if is_next { "flex items-center justify-end gap-2 text-sm opacity-70 mb-2" } else { "flex items-center gap-2 text-sm opacity-70 mb-2" },
                    if !is_next {
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-4 w-4",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 19l-7-7 7-7"
                            }
                        }
                    }
                    span { "{direction}" }
                    if is_next {
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-4 w-4",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M9 5l7 7-7 7"
                            }
                        }
                    }
                }
                h3 {
                    class: if is_next { "card-title text-base text-right" } else { "card-title text-base" },
                    "{title}"
                }
            }
        }
    }
}
