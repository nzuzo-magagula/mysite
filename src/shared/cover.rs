//! Generated cover artwork.
//!
//! Articles rarely ship a thumbnail, and the site used to fall back on a
//! single stock icon for all of them. These components derive a unique,
//! deterministic composition from a seed instead, so every card looks like
//! its own thing without any image assets being downloaded.

use dioxus::prelude::*;

/// A tiny deterministic RNG so server and client render byte-identical SVG.
struct Rng(u32);

impl Rng {
    fn seeded(text: &str) -> Self {
        // FNV-1a over the seed string.
        let mut h: u32 = 2_166_136_261;
        for b in text.as_bytes() {
            h ^= *b as u32;
            h = h.wrapping_mul(16_777_619);
        }
        Rng(h | 1)
    }

    fn from_u32(n: u32) -> Self {
        Rng(n.wrapping_mul(2_654_435_761) | 1)
    }

    /// xorshift32 → f32 in [0, 1)
    fn next(&mut self) -> f32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        (self.0 >> 8) as f32 / 16_777_216.0
    }

    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.next() * (hi - lo)
    }
}

/// Layered contour ribbons — the fallback cover for articles.
#[component]
pub fn CoverArt(seed: String, accent: String) -> Element {
    const W: f32 = 200.0;
    const H: f32 = 150.0;
    const BANDS: usize = 7;

    let mut rng = Rng::seeded(&seed);

    // Each band is a sine sweep with its own amplitude, phase and drift.
    let bands: Vec<(String, f32, f32)> = (0..BANDS)
        .map(|i| {
            let t = i as f32 / (BANDS - 1) as f32;
            let base = 24.0 + t * (H - 40.0);
            let amp = rng.range(5.0, 17.0);
            let phase = rng.range(0.0, 6.28);
            let freq = rng.range(1.1, 2.4);

            // Sample the curve and emit a smooth polyline.
            let mut d = String::with_capacity(220);
            for step in 0..=16 {
                let x = (step as f32 / 16.0) * W;
                let y = base + (phase + (x / W) * freq * 6.28).sin() * amp;
                d.push_str(if step == 0 { "M" } else { "L" });
                d.push_str(&format!("{x:.1},{y:.1} "));
            }

            let opacity = 0.16 + (1.0 - t) * 0.34;
            let width = 0.9 + rng.range(0.0, 1.5);
            (d, opacity, width)
        })
        .collect();

    // A few accent nodes scattered along the top band.
    let dots: Vec<(f32, f32, f32)> = (0..5)
        .map(|_| {
            (
                rng.range(12.0, W - 12.0),
                rng.range(14.0, H - 14.0),
                rng.range(1.4, 3.2),
            )
        })
        .collect();

    rsx! {
        div {
            class: "absolute inset-0 overflow-hidden",
            style: "background: linear-gradient(150deg, color-mix(in oklab, {accent} 13%, transparent), color-mix(in oklab, var(--color-secondary) 8%, transparent));",
            svg {
                class: "absolute inset-0 w-full h-full transition-transform duration-[1100ms] ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-105",
                view_box: "0 0 200 150",
                preserve_aspect_ratio: "xMidYMid slice",
                xmlns: "http://www.w3.org/2000/svg",
                "aria-hidden": "true",

                for (i , (d , opacity , width)) in bands.iter().enumerate() {
                    path {
                        key: "{i}",
                        d: "{d}",
                        fill: "none",
                        stroke: "{accent}",
                        stroke_width: "{width}",
                        stroke_opacity: "{opacity}",
                        stroke_linecap: "round",
                    }
                }

                for (i , (cx , cy , r)) in dots.iter().enumerate() {
                    circle {
                        key: "d{i}",
                        cx: "{cx}",
                        cy: "{cy}",
                        r: "{r}",
                        fill: "var(--color-accent)",
                        fill_opacity: "0.55",
                    }
                }
            }
        }
    }
}

/// A drifting node graph — used for demo cards.
#[component]
pub fn NodeArt(seed: u32, accent: String) -> Element {
    let mut rng = Rng::from_u32(seed);

    let nodes: Vec<(f32, f32, f32)> = (0..11)
        .map(|i| {
            let col = (i % 4) as f32;
            let row = (i / 4) as f32;
            (
                12.0 + col * 26.0 + rng.range(0.0, 16.0),
                18.0 + row * 26.0 + rng.range(0.0, 14.0),
                1.6 + rng.range(0.0, 2.6),
            )
        })
        .collect();

    // Connect any pair close enough to read as an edge.
    let edges: Vec<(f32, f32, f32, f32, f32)> = nodes
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            nodes.iter().skip(i + 1).filter_map(move |b| {
                let dist = ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
                (dist < 34.0).then_some((a.0, a.1, b.0, b.1, 0.45 - dist / 140.0))
            })
        })
        .collect();

    rsx! {
        svg {
            class: "absolute inset-0 w-full h-full opacity-70 transition-transform duration-[900ms] ease-[cubic-bezier(.22,1,.36,1)] group-hover:scale-110",
            view_box: "0 0 112 92",
            preserve_aspect_ratio: "xMidYMid slice",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",

            for (i , (x1 , y1 , x2 , y2 , opacity)) in edges.iter().enumerate() {
                line {
                    key: "e{i}",
                    x1: "{x1}",
                    y1: "{y1}",
                    x2: "{x2}",
                    y2: "{y2}",
                    stroke: "{accent}",
                    stroke_width: "0.5",
                    stroke_opacity: "{opacity}",
                }
            }

            for (i , n) in nodes.iter().enumerate() {
                circle {
                    key: "n{i}",
                    cx: "{n.0}",
                    cy: "{n.1}",
                    r: "{n.2}",
                    fill: "{accent}",
                    fill_opacity: "{0.35 + (i % 3) as f32 * 0.2}",
                }
            }
        }
    }
}
