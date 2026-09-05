# nnvis — tiny neural network demo

A Svelte 5 app that trains a small multilayer perceptron **in the browser**,
using real Python (CPython compiled to WebAssembly via
[Pyodide](https://pyodide.org)) rather than a JavaScript reimplementation.

There is no backend. Nothing is uploaded, and hosting costs nothing beyond
serving the static bundle.

## How it fits together

| Piece | Role |
| --- | --- |
| `src/py/trainer.py` | The network: dataset generation, forward pass, backpropagation, mini-batch SGD. Pure standard library — no numpy. |
| `src/lib/worker.js` | Boots Pyodide in a Web Worker, loads `trainer.py`, and exchanges JSON with the UI thread. |
| `src/App.svelte` | Controls, stats, and the layout. Holds no model logic. |
| `src/lib/BoundaryCanvas.svelte` | Decision surface + training points, drawn to a canvas. |
| `src/lib/NetworkDiagram.svelte` | Live weight matrices as an SVG graph — thickness is `|w|`, colour is the sign. |
| `src/lib/LossChart.svelte` | Cross-entropy over epochs. |

### Design notes

- **Pyodide loads lazily.** The page renders immediately; the ≈5 MB runtime is
  only fetched when someone presses *Start Python engine*. It comes from the
  jsDelivr CDN, so the origin never serves it.
- **Pure Python, no numpy.** This keeps the download to Pyodide's core (numpy
  would add a package fetch) and keeps `trainer.py` readable — the page
  displays it verbatim, so it has to be worth reading.
- **Training runs in a Worker.** Backpropagation in interpreted Python is slow
  enough to drop frames on the main thread.
- **Theme follows the host page.** The demo is embedded in an iframe and
  mirrors the site's `paper` / `ink` palette, both by reading the parent
  element directly and by listening for a `THEME_CHANGE` message.

## Development

```sh
npm install
npm run dev      # http://localhost:5173
```

Note that in `dev` the fonts (`/fonts/*.woff2`) come from the parent site and
will 404; the demo falls back to system faces. Everything else works.

## Building

```sh
npm run build
```

Output goes to `../public/neuralnet/`, which the Dioxus app serves at
`/neuralnet/index.html` and embeds from `/demos/neural-net`. The built bundle
is committed, matching how `react_demo` ships `public/algovis`.

## Changing the network

Everything interesting is in `src/py/trainer.py`. It can be run and tested with
ordinary CPython, no browser involved:

```sh
cd src/py
python3 -c "
import trainer
t = trainer.Trainer(dataset='moons', hidden=(8, 8), lr=0.5)
for _ in range(30):
    loss, acc = t.train_epochs(2)
print(f'loss {loss:.3f}  accuracy {acc:.0%}')
"
```
