(function(){"use strict";var l=`"""
A tiny neural network, trained from scratch in the browser.

This is the real thing: no autograd library, no numpy, no server. Just a
multilayer perceptron, backpropagation written out by hand, and stochastic
gradient descent. It runs under Pyodide in a Web Worker, which is why the
page stays responsive while it trains.

Pure standard library is a deliberate choice — it keeps the download to
Pyodide's core (no numpy package fetch) and keeps this file readable, since
the page shows it to you verbatim.
"""

import math
import random

# --------------------------------------------------------------------------
# Toy datasets: 2-D points with a binary label, i.e. the classic
# "can it learn this shape?" problems.
# --------------------------------------------------------------------------


def make_dataset(kind="moons", n=220, noise=0.18, seed=7):
    rng = random.Random(seed)
    pts = []

    for i in range(n):
        if kind == "xor":
            x = rng.uniform(-1, 1)
            y = rng.uniform(-1, 1)
            label = 1 if x * y > 0 else 0

        elif kind == "circle":
            angle = rng.uniform(0, 2 * math.pi)
            # Half the points inside a disc, half in an outer ring.
            radius = rng.uniform(0, 0.45) if i % 2 == 0 else rng.uniform(0.62, 1.0)
            x = radius * math.cos(angle)
            y = radius * math.sin(angle)
            label = 1 if radius < 0.5 else 0

        elif kind == "spiral":
            branch = i % 2
            t = 1.2 * math.pi * (i / n) + branch * math.pi
            r = 0.08 + 0.92 * (i / n)
            x = r * math.cos(t)
            y = r * math.sin(t)
            label = branch

        else:  # "moons" — two interleaving half-circles
            branch = i % 2
            # Sweep each half-circle evenly so the two arcs stay well formed.
            t = math.pi * ((i // 2) / max(1, (n // 2) - 1))
            if branch == 0:
                x, y = math.cos(t), math.sin(t)
            else:
                x, y = 1.0 - math.cos(t), 0.5 - math.sin(t)
            # Centre the pair on the origin and fit it to the view box.
            x = (x - 0.5) * 0.85
            y = (y - 0.25) * 0.85
            label = branch

        x += rng.gauss(0, noise)
        y += rng.gauss(0, noise)
        pts.append((x, y, label))

    return pts


# --------------------------------------------------------------------------
# Activations
# --------------------------------------------------------------------------


def sigmoid(z):
    # Split on the sign to avoid overflowing exp() on large magnitudes.
    if z >= 0:
        return 1.0 / (1.0 + math.exp(-z))
    e = math.exp(z)
    return e / (1.0 + e)


ACTIVATIONS = {
    "tanh": (math.tanh, lambda a: 1.0 - a * a),
    "relu": (lambda z: z if z > 0 else 0.0, lambda a: 1.0 if a > 0 else 0.0),
    "sigmoid": (sigmoid, lambda a: a * (1.0 - a)),
}


# --------------------------------------------------------------------------
# The network
# --------------------------------------------------------------------------


class MLP:
    """A fully connected network with a single sigmoid output."""

    def __init__(self, layer_sizes, activation="tanh", seed=1):
        rng = random.Random(seed)
        self.sizes = layer_sizes
        self.act_name = activation
        self.act, self.act_prime = ACTIVATIONS[activation]

        # He/Xavier-ish scaling keeps early activations from saturating.
        self.weights = []
        self.biases = []
        for n_in, n_out in zip(layer_sizes[:-1], layer_sizes[1:]):
            scale = math.sqrt(2.0 / n_in)
            self.weights.append(
                [[rng.gauss(0, scale) for _ in range(n_in)] for _ in range(n_out)]
            )
            self.biases.append([0.0 for _ in range(n_out)])

    # -- forward ----------------------------------------------------------

    def forward(self, x):
        """Return the list of activations, layer by layer."""
        acts = [x]
        a = x
        last = len(self.weights) - 1

        for layer, (W, b) in enumerate(zip(self.weights, self.biases)):
            z = [
                sum(w_ij * a_j for w_ij, a_j in zip(row, a)) + b_i
                for row, b_i in zip(W, b)
            ]
            # Hidden layers use the chosen activation; the output is always a
            # sigmoid so it reads as a probability.
            a = [sigmoid(v) for v in z] if layer == last else [self.act(v) for v in z]
            acts.append(a)

        return acts

    def predict(self, x):
        return self.forward(x)[-1][0]

    # -- backward ---------------------------------------------------------

    def backprop(self, x, target):
        """Gradients of binary cross-entropy w.r.t. every weight and bias."""
        acts = self.forward(x)
        last = len(self.weights) - 1

        # For a sigmoid output under cross-entropy the two derivatives cancel
        # and the output delta collapses to (prediction - target).
        delta = [acts[-1][0] - target]

        grad_w = [None] * len(self.weights)
        grad_b = [None] * len(self.biases)

        for layer in range(last, -1, -1):
            a_prev = acts[layer]
            grad_w[layer] = [[d * a_j for a_j in a_prev] for d in delta]
            grad_b[layer] = list(delta)

            if layer == 0:
                break

            # Push the error back through this layer's weights.
            W = self.weights[layer]
            a_here = acts[layer]
            new_delta = []
            for j in range(len(a_here)):
                err = sum(W[i][j] * delta[i] for i in range(len(delta)))
                new_delta.append(err * self.act_prime(a_here[j]))
            delta = new_delta

        return grad_w, grad_b


# --------------------------------------------------------------------------
# Training loop
# --------------------------------------------------------------------------


class Trainer:
    def __init__(
        self,
        dataset="moons",
        hidden=(8, 8),
        activation="tanh",
        lr=0.35,
        noise=0.18,
        seed=7,
        n_points=220,
    ):
        self.data = make_dataset(dataset, n=n_points, noise=noise, seed=seed)
        self.net = MLP([2] + list(hidden) + [1], activation=activation, seed=seed + 1)
        self.lr = lr
        self.epoch = 0
        self.history = []
        self.rng = random.Random(seed + 2)

    def loss_and_accuracy(self):
        total, correct = 0.0, 0
        eps = 1e-9
        for x, y, label in self.data:
            p = self.net.predict([x, y])
            total -= label * math.log(p + eps) + (1 - label) * math.log(1 - p + eps)
            if (p >= 0.5) == (label == 1):
                correct += 1
        n = len(self.data)
        return total / n, correct / n

    def train_epochs(self, epochs=1, batch_size=16):
        """Run \`epochs\` passes of mini-batch SGD over the whole dataset."""
        for _ in range(epochs):
            order = list(range(len(self.data)))
            self.rng.shuffle(order)

            for start in range(0, len(order), batch_size):
                batch = order[start : start + batch_size]

                # Accumulate gradients across the mini-batch.
                acc_w = [
                    [[0.0] * len(row) for row in W] for W in self.net.weights
                ]
                acc_b = [[0.0] * len(b) for b in self.net.biases]

                for idx in batch:
                    x, y, label = self.data[idx]
                    gw, gb = self.net.backprop([x, y], float(label))
                    for l in range(len(acc_w)):
                        for i in range(len(acc_w[l])):
                            acc_b[l][i] += gb[l][i]
                            row, grow = acc_w[l][i], gw[l][i]
                            for j in range(len(row)):
                                row[j] += grow[j]

                # Step against the averaged gradient.
                scale = self.lr / len(batch)
                for l in range(len(acc_w)):
                    for i in range(len(acc_w[l])):
                        self.net.biases[l][i] -= scale * acc_b[l][i]
                        row, grow = self.net.weights[l][i], acc_w[l][i]
                        for j in range(len(row)):
                            row[j] -= scale * grow[j]

            self.epoch += 1

        loss, acc = self.loss_and_accuracy()
        self.history.append(loss)
        if len(self.history) > 400:
            self.history = self.history[-400:]
        return loss, acc

    # -- readouts for the UI ----------------------------------------------

    def decision_grid(self, res=42, extent=2.0):
        """Flat row-major list of P(class 1) over the viewing window."""
        out = []
        step = (2 * extent) / (res - 1)
        for r in range(res):
            y = -extent + r * step
            for c in range(res):
                x = -extent + c * step
                out.append(self.net.predict([x, y]))
        return out

    def points(self):
        return [[x, y, label] for (x, y, label) in self.data]

    def topology(self):
        """Layer sizes plus every weight, for drawing the network diagram."""
        return {
            "sizes": self.net.sizes,
            "weights": self.net.weights,
        }

    def state(self):
        loss, acc = self.loss_and_accuracy()
        return {
            "epoch": self.epoch,
            "loss": loss,
            "accuracy": acc,
            "history": self.history,
        }
`;const o="https://cdn.jsdelivr.net/pyodide/v0.28.3/full/";let s=null,r=!1;function e(n){self.postMessage(n)}async function d(){if(r){e({type:"ready"});return}try{e({type:"status",phase:"fetching",detail:"Fetching Pyodide runtime"});const{loadPyodide:n}=await import(`${o}pyodide.mjs`);e({type:"status",phase:"booting",detail:"Starting CPython"}),s=await n({indexURL:o}),e({type:"status",phase:"loading-module",detail:"Loading trainer.py"}),s.FS.writeFile("/home/pyodide/trainer.py",l,{encoding:"utf8"}),s.runPython(`
import sys, json
if "/home/pyodide" not in sys.path:
    sys.path.insert(0, "/home/pyodide")
import trainer
_session = {}
`),r=!0,e({type:"ready",version:s.version})}catch(n){e({type:"error",message:String(n&&n.message||n)})}}function c(n){const a=JSON.stringify(n),t=s.runPython(`
cfg = json.loads(${JSON.stringify(a)})
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
`);return JSON.parse(t)}function f(n,a,t){const i=s.runPython(`
t = _session["t"]
t.train_epochs(${n})
_payload = {"state": t.state()}
if ${t?"True":"False"}:
    _payload["grid"] = [round(v, 3) for v in t.decision_grid(${a})]
    _payload["topology"] = t.topology()
json.dumps(_payload)
`);return JSON.parse(i)}self.onmessage=async n=>{const a=n.data||{};try{switch(a.cmd){case"init":await d();break;case"build":if(!r)return e({type:"error",message:"Runtime not ready"});e({type:"built",...c(a.config)});break;case"lr":r&&s&&s.runPython(`
if "t" in _session:
    _session["t"].lr = ${Number(a.value)||.5}
`);break;case"train":{if(!r)return e({type:"error",message:"Runtime not ready"});const t=performance.now(),i=f(a.epochs,a.gridRes,a.wantGrid);e({type:"tick",...i,ms:performance.now()-t});break}default:break}}catch(t){e({type:"error",message:String(t&&t.message||t)})}}})();
