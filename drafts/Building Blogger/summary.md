#####
short_summary = "Building a fullstack blog in Dioxus: server-side rendering with client hydration, a markdown parser with TOML frontmatter, IndexedDB caching, GitHub integration, and getting the WASM bundle down to something reasonable. One Rust codebase, running on web, desktop and mobile."
name = "summary"
#####
# Building Blogger: A Fullstack Dioxus Application

This series documents building the blog application you're reading this on, using Dioxus. It's not a simplified tutorial project. It's the actual code, including the parts that took several attempts.

The application covers a fair amount of ground: multiple pages (home, article viewer, series listing, about), a custom markdown parser with TOML frontmatter and syntax highlighting, series auto-detected from folder structure, previous/next navigation and a table of contents, GitHub repository display with client-side caching, and a mobile-first responsive design with theme support.

Underneath that it does server-side rendering for fast initial loads, client-side hydration once the WASM arrives, hot reloading when articles change during development, type-safe RPC through server functions, IndexedDB caching via netabase_store, and enough WASM optimization to keep the production bundle under 500KB.

## What You Need to Know

You'll get the most out of this if you're comfortable with Rust fundamentals (ownership, traits, async), basic web concepts, and component-based UI frameworks like React or Vue.

Dioxus itself is explained as we go, along with the fullstack architecture patterns and the optimization techniques. You don't need those beforehand.

## The Articles

### Part 1: Introduction and Architecture

Why Dioxus for fullstack work, how the project is organized, type-safe routing, server functions as an alternative to REST APIs, signal-based reactivity, and feature-gated compilation.

The core idea is that a server function looks like a normal Rust call from the client:

```rust
// Server function that runs on server
#[server]
async fn fetch_article(path: String) -> Result<Article> {
    tokio::fs::read_to_string(&path).await
}

// Called from client - looks like normal Rust
let article = fetch_article("post.md".to_string()).await?;
```

### Part 2: Markdown Parser and Rendering

Parsing TOML frontmatter, integrating `dioxus_markdown`, adding syntax highlighting with Prism.js, custom markdown extensions, and the rendering pipeline.

The frontmatter parsing is deliberately simple:

```rust
/// Parse TOML metadata between ##### delimiters
fn parse_toml_metadata(content: &str) -> Option<ArticleTomlMetadata> {
    const DELIMITER: &str = "#####";

    let first_pos = content.find(DELIMITER)?;
    let after_first = &content[first_pos + DELIMITER.len()..];
    let second_pos = after_first.find(DELIMITER)?;
    let toml_content = &after_first[..second_pos].trim();

    toml::from_str(toml_content).ok()
}
```

Which parses metadata blocks that look like this:

```toml
#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Article summary for SEO"
thumbnail = "https://..."
category = "Technical"
tags = ["rust", "dioxus", "tutorial"]

[[article_series]]
name = "Building Blogger"
previous = "Building Blogger/01-introduction"
next = "Building Blogger/03-article-management"
#####
```

Also covered: markdown AST manipulation, math rendering with KaTeX, custom block types like callouts and code tabs, and generating SEO metadata.

### Part 3: Article Management and Caching

How articles are laid out on disk, detecting series from folder structure, extracting titles without parsing the whole file, server-side caching with the `cached` crate, client-side caching in IndexedDB, and file watching for hot reload.

The layout is just directories:

```
articles/
├── Building Blogger/
│   ├── 01-introduction.md
│   ├── 02-markdown.md
│   ├── summary.md           # Special file
│   └── assets/
└── netabase_store/
    ├── 01-introduction.md
    ├── 02-macros.md
    └── ...
```

GitHub data gets cached in the browser through netabase_store:

```rust
#[derive(NetabaseModel, bincode::Encode, bincode::Decode)]
#[netabase(GitHubCacheDefinition)]
pub struct CachedGitHubData {
    #[primary_key]
    pub cache_key: String,
    pub repos: Vec<GitHubRepo>,
    pub cached_at: f64, // JavaScript timestamp
}

// Usage
async fn fetch_repos_cached() -> Result<Vec<GitHubRepo>> {
    // Check cache
    if let Some(cached) = get_cached().await? {
        if !is_stale(cached.cached_at) {
            return Ok(cached.repos);
        }
    }

    // Fetch fresh
    let repos = fetch_from_github().await?;
    save_to_cache(&repos).await?;

    Ok(repos)
}
```

Which gets into cache invalidation, file watching with `notify`, and GitHub's rate limits.

### Part 4: Routing and Page Components

Type-safe routing with the `Routable` derive, dynamic segments and catch-alls, component props and state, loading states and error handling, navigation components, and responsive layouts.

Routes are an enum:

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    // Catch-all for nested paths
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },

    #[route("/series")]
    Series {},

    #[route("/series/:name")]
    SeriesDetail { name: String },
}
```

And pages are components that match on resource state:

```rust
#[component]
pub fn ArticlePage(path: String) -> Element {
    // Reactive resource - refetches when path changes
    let article_data = use_resource(move || async move {
        fetch_article_with_metadata(path.clone()).await
    });

    rsx! {
        div {
            match article_data.read().as_ref() {
                Some(Ok(article)) => rsx! {
                    ArticleMetadata { meta: article.toml_metadata }
                    Markdown { content: article.content }
                    NavigationCards { meta: article.toml_metadata }
                },
                Some(Err(e)) => rsx! {
                    ErrorDisplay { error: e.to_string() }
                },
                None => rsx! {
                    LoadingSkeleton {}
                }
            }
        }
    }
}
```

Also: error boundaries, loading skeletons, accessibility, and mobile-first design.

### Part 5: Fullstack Architecture and Optimization

Server functions in depth, the SSR versus CSR trade-off, WASM bundle optimization, code splitting, deployment, and monitoring.

The optimization config that got the bundle down:

```toml
[profile.release]
opt-level = 'z'         # Optimize for size
lto = true              # Link Time Optimization
codegen-units = 1       # Better optimization
panic = 'abort'         # Reduce binary size
strip = "debuginfo"     # Strip debug info

[web.wasm-opt]
level = 'z'             # Maximum compression

[web.pre-compress]
enabled = true          # Brotli/gzip
```

The difference it makes:

| Metric | Dev Mode | Release Mode |
|--------|----------|--------------|
| WASM Size | ~8MB | ~500KB |
| First Paint | ~53s | ~2s |
| Interaction | ~55s | ~2.5s |
| Bundle Load | ~50s | ~1s |

Server functions come in a few shapes, from plain fetching to cached to authenticated:

```rust
// Simple data fetching
#[server]
async fn fetch_article(path: String) -> Result<String> {
    tokio::fs::read_to_string(&path).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

// With caching
#[server]
#[cached(time = 300, result = true)]  // Cache for 5 minutes
async fn fetch_article_cached(path: String) -> Result<String> {
    fetch_article(path).await
}

// With authentication
#[server]
async fn update_article(
    path: String,
    content: String,
    auth: String
) -> Result<()> {
    verify_auth(&auth)?;
    tokio::fs::write(&path, content).await?;
    Ok(())
}
```

Plus bundle analysis, dynamic imports, PWA features, Docker deployment, CI/CD, SEO, and performance budgets.

## Recurring Patterns

A handful of things come up repeatedly across the series.

On the Dioxus side: component composition, signal-based reactivity and effects, resource lifecycle, context for global state, and conditional rendering.

On the fullstack side: server functions for RPC, type-safe client-server communication, feature-gating code per platform, SSR with hydration, and progressive enhancement.

On performance: lazy loading, code splitting, aggressive WASM optimization, multi-level caching, efficient filesystem operations, and minimizing re-renders.

And architecturally: module organization, separation of concerns, dependency injection, error handling, and testing.

## Project Structure

```
blogger/
├── src/
│   ├── main.rs                 # Entry point, routing
│   ├── markdown_management/    # Content management
│   │   ├── mod.rs
│   │   ├── local.rs            # File system ops
│   │   ├── github.rs           # GitHub API
│   │   ├── github_cache.rs     # IndexedDB cache (web)
│   │   └── watcher.rs          # File watching (server)
│   ├── pages/                  # Page components
│   │   ├── home_page/
│   │   │   ├── mod.rs
│   │   │   ├── blog.rs         # Blog section
│   │   │   └── projects.rs     # Projects section
│   │   ├── article_page/
│   │   │   └── mod.rs
│   │   ├── series_page.rs
│   │   ├── series_detail_page.rs
│   │   ├── about_page.rs
│   │   ├── demos_page.rs
│   │   └── reading_page.rs
│   └── shared/                 # Shared components
│       ├── mod.rs
│       └── nav_bar.rs
├── articles/                   # Markdown content
│   ├── Building Blogger/
│   ├── netabase_store/
│   └── Project Management/
├── assets/                     # Static files
│   └── tailwind.css
├── Cargo.toml                  # Dependencies
├── Dioxus.toml                 # Dioxus configuration
└── PERFORMANCE.md              # Performance notes
```

The dependencies are fairly small:

```toml
[dependencies]
dioxus = { version = "0.7.0-rc-3", features = ["router", "fullstack"] }
dioxus_markdown = "..." # Markdown rendering
tokio = "1.48.0"       # Async runtime (server)
serde = "1.0"          # Serialization
anyhow = "1.0"         # Error handling
cached = "0.54"        # Server-side caching
notify = "7.0"         # File watching
netabase_store = "..." # IndexedDB (web)
```

And the workflow:

```bash
# Start dev server
dx serve

# In another terminal, watch for changes
dx watch

# Build for production
dx build --release

# Deploy
docker build -t blogger .
docker run -p 8080:8080 blogger
```

## Three Patterns Worth Memorizing

Fetching data:

```rust
// Define server function
#[server]
async fn fetch_data() -> Result<Data> {
    // Server-side logic
}

// Use in component
let data = use_resource(|| async move {
    fetch_data().await
});

// Render
match data.read().as_ref() {
    Some(Ok(d)) => rsx! { /* render data */ },
    Some(Err(e)) => rsx! { /* error */ },
    None => rsx! { /* loading */ },
}
```

Navigation:

```rust
let nav = navigator();

// Navigate programmatically
nav.push(Route::Article {
    segments: vec!["post".to_string()]
});

// Or use Link component
rsx! {
    Link {
        to: Route::Article { segments: vec!["post".to_string()] },
        "Read Article"
    }
}
```

State:

```rust
// Local state
let mut count = use_signal(|| 0);

// Global state with context
let ctx: MyContext = use_context();

// Derived state
let doubled = use_memo(move || count() * 2);

// Effects
use_effect(move || {
    println!("Count: {}", count());
});
```

## Where to Start

[Part 1: Introduction and Architecture Overview](./01-introduction-and-architecture.md)

---

This blog runs in production at NewsNet Africa, publishing technical articles, tutorials and project documentation. It's a reasonable argument that Rust and Dioxus are ready for real web work.

Source code: [github.com/nzuzo-newsnet/blogger](https://github.com/nzuzo-newsnet/blogger)

Dioxus: [dioxuslabs.com](https://dioxuslabs.com)

Articles are CC BY-SA 4.0, code is MIT OR Apache-2.0.
