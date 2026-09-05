/*
 * Pyodide worker.
 *
 * Training happens on a background thread so the main thread can keep
 * painting at 60fps while Python grinds through backpropagation. Everything
 * crossing the boundary is plain JSON — Python hands back `json.dumps`
 * strings rather than PyProxy objects, which sidesteps manual proxy
 * lifetime management entirely.
 */

import trainerSource from "../py/trainer.py?raw";

const PYODIDE_VERSION = "0.28.3";
const PYODIDE_BASE = `https://cdn.jsdelivr.net/pyodide/v${PYODIDE_VERSION}/full/`;

let pyodide = null;
let ready = false;

function post(msg) {
  self.postMessage(msg);
}

async function init() {
  if (ready) {
    post({ type: "ready" });
    return;
  }

  try {
    post({ type: "status", phase: "fetching", detail: "Fetching Pyodide runtime" });

    // Loaded from the CDN at runtime, never bundled — that is what keeps this
    // demo free to host.
    const { loadPyodide } = await import(
      /* @vite-ignore */ `${PYODIDE_BASE}pyodide.mjs`
    );

    post({ type: "status", phase: "booting", detail: "Starting CPython" });

    pyodide = await loadPyodide({ indexURL: PYODIDE_BASE });

    post({ type: "status", phase: "loading-module", detail: "Loading trainer.py" });

    // Register trainer.py as a real importable module.
    pyodide.FS.writeFile("/home/pyodide/trainer.py", trainerSource, {
      encoding: "utf8",
    });
    pyodide.runPython(`
import sys, json
if "/home/pyodide" not in sys.path:
    sys.path.insert(0, "/home/pyodide")
import trainer
_session = {}
`);

    ready = true;
    post({ type: "ready", version: pyodide.version });
  } catch (err) {
    post({ type: "error", message: String((err && err.message) || err) });
  }
}

function build(config) {
  const cfg = JSON.stringify(config);
  const raw = pyodide.runPython(`
cfg = json.loads(${JSON.stringify(cfg)})
_session["t"] = trainer.Trainer(
    dataset=cfg["dataset"],
    hidden=tuple(cfg["hidden"]),
    activation=cfg["activation"],
    lr=cfg["lr"],
    noise=cfg["noise"],
    seed=cfg["seed"],
    n_points=cfg["points"],
)
t = _session["t"]
json.dumps({
    "points": [[round(x, 4), round(y, 4), c] for x, y, c in t.points()],
    "topology": t.topology(),
    "grid": [round(v, 3) for v in t.decision_grid(cfg["gridRes"])],
    "state": t.state(),
})
`);
  return JSON.parse(raw);
}

function train(epochs, gridRes, wantGrid) {
  const raw = pyodide.runPython(`
t = _session["t"]
t.train_epochs(${epochs})
_payload = {"state": t.state()}
if ${wantGrid ? "True" : "False"}:
    _payload["grid"] = [round(v, 3) for v in t.decision_grid(${gridRes})]
    _payload["topology"] = t.topology()
json.dumps(_payload)
`);
  return JSON.parse(raw);
}

self.onmessage = async (event) => {
  const msg = event.data || {};

  try {
    switch (msg.cmd) {
      case "init":
        await init();
        break;

      case "build":
        if (!ready) return post({ type: "error", message: "Runtime not ready" });
        post({ type: "built", ...build(msg.config) });
        break;

      case "lr":
        // Learning rate is the one knob that can change without rebuilding.
        if (ready && pyodide) {
          pyodide.runPython(
            `\nif "t" in _session:\n    _session["t"].lr = ${Number(msg.value) || 0.5}\n`
          );
        }
        break;

      case "train": {
        if (!ready) return post({ type: "error", message: "Runtime not ready" });
        const t0 = performance.now();
        const out = train(msg.epochs, msg.gridRes, msg.wantGrid);
        post({ type: "tick", ...out, ms: performance.now() - t0 });
        break;
      }

      default:
        break;
    }
  } catch (err) {
    post({ type: "error", message: String((err && err.message) || err) });
  }
};
