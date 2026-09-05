# nzuzomagagula.online

Personal site and writing archive. A Rust application compiled to WebAssembly
with [Dioxus](https://dioxuslabs.com), serving Markdown articles from disk and
embedding a couple of self-contained interactive demos.

## Layout

```
.
├─ src/
│  ├─ main.rs                  # routes, theme boot, ambient background, motion runtime
│  ├─ shared/
│  │  ├─ nav_bar.rs            # navigation + light/dark toggle
│  │  └─ cover.rs              # generated cover artwork (no stock imagery)
│  ├─ pages/                   # one module per route
│  └─ markdown_management/     # article loading, GitHub fetch, caching, file watcher
├─ articles/                   # content — folders become series, loose files standalone
├─ aboutme.md                  # the About page body
├─ tailwind.css                # design system source  ──┐ compiled to
├─ assets/tailwind.css         # committed build output ─┘
├─ public/                     # served at the site root
│  ├─ fonts/                   # self-hosted variable fonts
│  ├─ js/motion.js             # scroll reveals, pointer effects, reading progress
│  ├─ algovis/                 # built output of react_demo
│  └─ neuralnet/               # built output of svelte_demo
├─ react_demo/                 # algorithm visualiser (React)
└─ svelte_demo/                # neural network trainer (Svelte + Python)
```

## Design system

Colour, type and motion are defined once in `tailwind.css` and consumed
everywhere else through tokens.

- **Two hand-built themes**, `paper` (light) and `ink` (dark), declared with
  `@plugin "daisyui/theme"` over a hand-picked OKLCH palette. daisyUI's own
  presets are switched off (`themes: false`) — it is used for component
  primitives only.
- **Self-hosted type**: Fraunces (display), Manrope (text), JetBrains Mono
  (code), latin subsets only, ~123 KB total. No third-party font requests.
- **Tokens** — `--surface`, `--hairline`, `--elev-1..3`, `--radius-*` — rather
  than per-component colour choices. Component classes (`.surface`, `.chip`,
  `.pill`, `.btn-solid`, `.card-interactive`) build on them.
- **Motion** is opt-in per element via `data-reveal`, driven by
  `public/js/motion.js`, and fully disabled under
  `prefers-reduced-motion: reduce`.

The theme lives on `<html data-theme>`, is persisted in `localStorage`, and is
broadcast to embedded demo iframes via `postMessage`.

> Adding a theme colour means editing the two `@plugin "daisyui/theme"` blocks
> in `tailwind.css` **and** the matching blocks in `react_demo/src/index.css`
> and `svelte_demo/src/app.css`, which the iframes use to mirror the palette.

## Running it

```bash
dx serve --platform web
```

### Rebuilding the stylesheet

`assets/tailwind.css` is committed build output. Because the stylesheet uses
daisyUI as a plugin, regenerate it with the Tailwind CLI rather than relying on
Dioxus' built-in pass:

```bash
npm install
npx @tailwindcss/cli -i tailwind.css -o assets/tailwind.css   # add --watch while working
```

### Rebuilding the demos

Both demos are separate apps whose built output is committed under `public/`,
so a plain `dx build` needs no Node toolchain.

```bash
cd react_demo  && npm install && npm run build   # -> public/algovis
cd svelte_demo && npm install && npm run build   # -> public/neuralnet
```

See `svelte_demo/README.md` for how the Python trainer works and how to
exercise it without a browser.

## Content

Markdown files in `articles/` are picked up automatically; in development a
file watcher reloads them without a restart. A folder becomes a series, and its
`summary.md` provides the series description. TOML frontmatter supplies title,
date, category, topics, reading time and thumbnail — see
`TOML_CONFIGURATION_GUIDE.md` and `MARKDOWN_FEATURES.md`.

## Deploying

`Dockerfile` builds the Rust app and copies `public/`, `articles/` and
`aboutme.md` into a slim runtime image listening on `$PORT` (default 8080).

## Contact

- General — me@nzuzomagagula.online
- Development — dev@nzuzomagagula.online
