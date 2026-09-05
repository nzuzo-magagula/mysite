#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "The architecture behind this blog: Dioxus server functions, type-safe routing, signal-based state, and one codebase compiling for server and browser"
thumbnail = "https://i.postimg.cc/pdKhS5Rk/blogger-architecture.png"
category = "Technical"
show_references = true

[[article_series]]
name = "Building Blogger"
next = "Building Blogger/02-markdown-parser-and-rendering"
#####
# Building Blogger: Introduction and Architecture

This series walks through building a complete blog application in Dioxus. Not a simplified example, the actual production code, which means fullstack architecture with server-side rendering and client hydration, a markdown parser with TOML metadata and series support, IndexedDB caching through netabase_store, file system watching for auto-reload, GitHub repository display with API caching, Tailwind with theme support, and type safety from server to client.

## Why Dioxus?

Dioxus brings React-like ergonomics to Rust, and a few things make it worth using here.

### One component, several targets

```rust
#[component]
fn MyComponent(name: String) -> Element {
    rsx! {
        div { "Hello, {name}!" }
    }
}
```

That component runs on the web compiled to WASM, on desktop through a webview, on iOS and Android, and on the server for SSR.

### Server functions

```rust
#[server]
async fn fetch_article(path: String) -> Result<Article, ServerError> {
    // This runs on the server only
    let content = tokio::fs::read_to_string(&path).await?;
    Ok(parse_article(content))
}

// Call from client code
let article = fetch_article("post.md".to_string()).await?;
```

The `#[server]` macro generates the server-side implementation, the client-side RPC stub, the serialization, and the type-safe glue between them.

### Signal-based reactivity

```rust
let mut count = use_signal(|| 0);

rsx! {
    button {
        onclick: move |_| count += 1,
        "Clicked {count} times"
    }
}
```

Signals give you fine-grained updates, minimal re-renders, and no lifecycle hooks to reason about.

## Project Architecture

```
blogger/
├── src/
│   ├── main.rs                     # Entry point and routing
│   ├── markdown_management/        # Article and content management
│   │   ├── local.rs                # File system operations
│   │   ├── github.rs               # GitHub API integration
│   │   ├── github_cache.rs         # IndexedDB caching (WASM)
│   │   └── watcher.rs              # File watching (server)
│   ├── pages/                      # Page components
│   │   ├── home_page/              # Landing page
│   │   ├── article_page/           # Article viewer
│   │   ├── series_page.rs          # Series listing
│   │   └── ...
│   └── shared/                     # Shared components
│       └── nav_bar.rs              # Navigation
├── articles/                       # Markdown content
│   ├── netabase_store/             # Tutorial series
│   ├── Project Management/         # Another series
│   └── ...
└── assets/                         # Static assets
```

### The entry point

```rust
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/article/:..segments")]
    Article { segments: Vec<String> },
    #[route("/series")]
    Series {},
    #[route("/series/:name")]
    SeriesDetail { name: String },
    // ...
}

fn main() {
    // Start file watcher on server
    #[cfg(feature = "server")]
    {
        if let Err(e) = markdown_management::start_article_watcher() {
            logger::error!("Failed to start watcher: {}", e);
        }
    }

    dioxus::launch(App);
}
```

Routing is type-safe through the `Routable` derive, catch-all segments handle nested paths, conditional compilation splits server from client, and the file watcher only exists on the server side.

### Markdown management

This module handles all the content operations:

```rust
// Server-side: Read files from disk
#[server]
pub async fn fetch_article_with_metadata(
    path: String
) -> Result<ArticleWithMetadata, ServerFnError> {
    let content = tokio::fs::read_to_string(&full_path).await?;
    let toml_metadata = parse_toml_metadata(&content);

    Ok(ArticleWithMetadata {
        metadata: ArticleMetadata {
            name: filename,
            path: relative_path,
            title: extract_title(&content),
        },
        toml_metadata,
        content: extract_content_without_metadata(&content),
    })
}
```

The metadata itself is a plain struct:

```rust
#[derive(Serialize, Deserialize)]
pub struct ArticleTomlMetadata {
    pub date: Option<String>,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub thumbnail: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub article_series: Vec<ArticleSeries>,
    // ... more fields
}
```

Which comes from TOML frontmatter in each article:

```markdown
#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Article summary"
category = "Technical"

[[article_series]]
name = "Building Blogger"
next = "Building Blogger/02-markdown-parser"
#####
# Article Title

Content here...
```

### Page components

Each page is a Dioxus component:

```rust
#[component]
pub fn ArticlePage(path: String) -> Element {
    // Fetch article data
    let article_data = use_resource(move || async move {
        fetch_article_with_metadata(path.clone()).await
    });

    rsx! {
        main {
            class: "container",
            match article_data.read().as_ref() {
                Some(Ok(article)) => rsx! {
                    Markdown { content: article.content.clone() }
                    NavigationCards { metadata: article.toml_metadata }
                },
                Some(Err(e)) => rsx! {
                    div { "Error: {e}" }
                },
                None => rsx! {
                    div { "Loading..." }
                }
            }
        }
    }
}
```

`use_resource` handles the async fetch, you pattern match on the resource state, it re-fetches automatically when dependencies change, and props are type-checked.

## Design Decisions

### Server functions instead of REST

The traditional version means defining endpoints and then constructing calls to them:

```rust
// Traditional approach
#[get("/api/article/<path>")]
async fn get_article(path: String) -> Json<Article> {
    // Implementation
}

// Client side
let response = fetch("/api/article/post.md").await?;
let article: Article = response.json().await?;
```

The Dioxus version collapses that:

```rust
#[server]
async fn fetch_article(path: String) -> Result<Article, ServerFnError> {
    // Implementation
}

// Client side (identical to server!)
let article = fetch_article("post.md".to_string()).await?;
```

No URL construction, no manual serialization, type-safe RPC, and shared code between the two sides.

### Feature-gated code

The same codebase compiles for different platforms:

```rust
// Always compiled
pub struct ArticleMetadata {
    pub name: String,
    pub path: String,
}

// Server only
#[cfg(feature = "server")]
async fn read_from_filesystem(path: &str) -> Result<String> {
    tokio::fs::read_to_string(path).await
}

// Web only
#[cfg(feature = "web")]
async fn read_from_indexeddb(key: &str) -> Result<String> {
    // IndexedDB operations
}
```

Which means conditional dependencies (no tokio in WASM), platform-specific optimizations, and one codebase for every target.

### Signals over useState and useEffect

```rust
// Create reactive signal
let mut count = use_signal(|| 0);

// Read the value
let current = count();

// Update the value
count += 1;

// Derived computation
let doubled = use_memo(move || count() * 2);

// Effects
use_effect(move || {
    println!("Count changed to: {}", count());
});
```

Dependency tracking is automatic, there are no dependency arrays, reactivity is fine-grained, and signals are `Copy`, so cloning is cheap.

### Articles organized by folder

```
articles/
├── Building Blogger/
│   ├── 01-introduction.md
│   ├── 02-markdown.md
│   └── summary.md
└── netabase_store/
    ├── 01-introduction.md
    ├── 02-macros.md
    └── summary.md
```

Series get detected from folder names, navigation comes from the TOML metadata, series pages generate themselves, and multi-level navigation works out of that.

## Data Flow

An initial request goes browser to server, reads the filesystem, parses the markdown, renders HTML, and returns it. The browser then loads the WASM, hydrates the components, attaches event handlers, and the page becomes interactive. Subsequent navigation happens client-side, calling server functions and updating the UI without a full page load.

GitHub data gets cached in the browser:

```rust
#[cfg(feature = "web")]
pub async fn fetch_github_repos_cached() -> Result<Vec<GitHubRepo>> {
    // Try cache first
    if let Some(cached) = get_from_indexeddb("github_repos").await? {
        if !is_stale(cached.cached_at) {
            return Ok(cached.repos);
        }
    }

    // Fetch fresh data
    let repos = fetch_github_repos().await?;

    // Update cache
    save_to_indexeddb("github_repos", &repos).await?;

    Ok(repos)
}
```

That uses `netabase_store` for typed IndexedDB access:

```rust
#[derive(NetabaseModel, bincode::Encode, bincode::Decode)]
#[netabase(GitHubCacheDefinition)]
pub struct CachedGitHubData {
    #[primary_key]
    pub cache_key: String,
    pub repos: Vec<GitHubRepo>,
    pub cached_at: f64, // Timestamp
}
```

## Performance

The WASM bundle needs aggressive optimization to be usable:

```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link Time Optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Reduce binary size
strip = "debuginfo" # Strip debug info

[web.wasm-opt]
level = 'z'         # Maximum compression
```

A dev build is around 8MB of WASM. The release build is around 500KB gzipped, with first contentful paint under 2 seconds.

Resources load on demand:

```rust
// Article only loads when navigated to
let article = use_resource(move || async move {
    fetch_article(path()).await
});

// GitHub repos load in background
let repos = use_resource(|| async move {
    fetch_github_repos_cached().await
});
```

And in development, articles reload without restarting the server:

```rust
#[cfg(feature = "server")]
pub fn start_article_watcher() -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        tx.send(res).unwrap();
    })?;

    watcher.watch("./articles", RecursiveMode::Recursive)?;

    tokio::spawn(async move {
        while let Ok(event) = rx.recv() {
            // Clear cache
            ARTICLE_CACHE.clear();
        }
    });

    Ok(())
}
```

## What's Coming

Part 2 covers the markdown parser: custom extensions, TOML frontmatter parsing, syntax highlighting, math rendering, and component integration.

Part 3 covers article management and caching: file system organization, metadata extraction, series detection, IndexedDB, and the GitHub API.

Part 4 covers routing and page components: type-safe routing, dynamic segments, nested routes, navigation, and loading states.

Part 5 covers fullstack architecture and optimization: server function patterns, the SSR versus CSR trade-off, bundle size, caching strategies, and deployment.

## Following Along

You'll need Rust 1.75 or later, Node and npm for Tailwind, and the Dioxus CLI:

```bash
cargo install dioxus-cli
dx new my-blog --template=fullstack
cd my-blog
dx serve
```

That gets you a running app on http://localhost:8080.

Next article: the markdown rendering system, and how TOML metadata, syntax highlighting and the rendering pipeline fit together.

---

Project repository: [github.com/nzuzo-newsnet/blogger](https://github.com/nzuzo-newsnet/blogger)

Dioxus documentation: [dioxuslabs.com](https://dioxuslabs.com)

Further reading: the [Dioxus Book](https://dioxuslabs.com/learn/0.7/), the [server functions guide](https://dioxuslabs.com/learn/0.7/reference/server_functions), and the [fullstack apps cookbook](https://dioxuslabs.com/learn/0.7/cookbook/fullstack).
