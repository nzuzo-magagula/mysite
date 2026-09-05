<script>
  import BoundaryCanvas from "./lib/BoundaryCanvas.svelte";
  import NetworkDiagram from "./lib/NetworkDiagram.svelte";
  import LossChart from "./lib/LossChart.svelte";
  import trainerSource from "./py/trainer.py?raw";
  import PyWorker from "./lib/worker.js?worker";

  const GRID_RES = 42;
  const EPOCHS_PER_TICK = 2;

  // ---- runtime state -----------------------------------------------------
  let worker = null;
  let phase = $state("idle"); // idle | loading | ready | error
  let statusLine = $state("");
  let errorMsg = $state("");
  let pyVersion = $state("");

  // ---- model state -------------------------------------------------------
  let points = $state([]);
  let grid = $state([]);
  let topology = $state(null);
  let epoch = $state(0);
  let loss = $state(null);
  let accuracy = $state(null);
  let history = $state([]);
  let lastMs = $state(0);

  // ---- controls ----------------------------------------------------------
  let dataset = $state("moons");
  let hiddenSpec = $state("8,8");
  let activation = $state("tanh");
  let lr = $state(0.5);
  let noise = $state(0.15);
  let running = $state(false);
  let busy = false; // a train message is in flight
  let showCode = $state(false);

  const DATASETS = [
    { id: "moons", label: "Two moons" },
    { id: "circle", label: "Circle" },
    { id: "xor", label: "XOR" },
    { id: "spiral", label: "Spiral" },
  ];

  const ACTIVATIONS = ["tanh", "relu", "sigmoid"];

  function parseHidden(spec) {
    const layers = spec
      .split(",")
      .map((s) => parseInt(s.trim(), 10))
      .filter((n) => Number.isFinite(n) && n >= 1 && n <= 16);
    return layers.length ? layers.slice(0, 3) : [8, 8];
  }

  function config() {
    return {
      dataset,
      hidden: parseHidden(hiddenSpec),
      activation,
      lr: Number(lr),
      noise: Number(noise),
      seed: 7,
      points: 220,
      gridRes: GRID_RES,
    };
  }

  // ---- worker plumbing ---------------------------------------------------

  function startRuntime() {
    if (phase === "loading" || phase === "ready") return;
    phase = "loading";
    statusLine = "Contacting CDN…";
    errorMsg = "";

    worker = new PyWorker();
    worker.onmessage = (e) => handle(e.data);
    worker.onerror = (e) => {
      phase = "error";
      errorMsg = e.message || "Worker failed to start";
    };
    worker.postMessage({ cmd: "init" });
  }

  function handle(msg) {
    switch (msg.type) {
      case "status":
        statusLine = msg.detail;
        break;

      case "ready":
        phase = "ready";
        pyVersion = msg.version || "";
        statusLine = "";
        // The config effect below reacts to `phase` and issues the first
        // build, so doing it here too would build twice.
        break;

      case "built":
        points = msg.points;
        grid = msg.grid;
        topology = msg.topology;
        applyState(msg.state);
        busy = false;
        break;

      case "tick":
        if (msg.grid) grid = msg.grid;
        if (msg.topology) topology = msg.topology;
        applyState(msg.state);
        lastMs = msg.ms;
        busy = false;
        if (running) queueTick();
        break;

      case "error":
        phase = "error";
        errorMsg = msg.message;
        running = false;
        busy = false;
        break;
    }
  }

  function applyState(s) {
    epoch = s.epoch;
    loss = s.loss;
    accuracy = s.accuracy;
    history = s.history;
  }

  function rebuild() {
    if (phase !== "ready") return;
    running = false;
    busy = true;
    worker.postMessage({ cmd: "build", config: config() });
  }

  let tickHandle = 0;
  function queueTick() {
    cancelAnimationFrame(tickHandle);
    tickHandle = requestAnimationFrame(() => step(true));
  }

  function step(fromLoop = false) {
    if (phase !== "ready" || busy) return;
    if (!fromLoop) running = false;
    busy = true;
    worker.postMessage({
      cmd: "train",
      epochs: EPOCHS_PER_TICK,
      gridRes: GRID_RES,
      wantGrid: true,
    });
  }

  function toggleRun() {
    if (phase !== "ready") return;
    running = !running;
    if (running) step(true);
  }

  // Any change to the architecture or data invalidates the current model.
  $effect(() => {
    dataset;
    hiddenSpec;
    activation;
    noise;
    if (phase === "ready") rebuild();
  });

  // Learning rate can change mid-flight without a rebuild.
  $effect(() => {
    if (phase === "ready" && worker) {
      worker.postMessage({ cmd: "lr", value: Number(lr) });
    }
  });

  // ---- theme sync with the host page -------------------------------------
  $effect(() => {
    function applyTheme(t) {
      if (t === "paper" || t === "ink") {
        document.documentElement.setAttribute("data-theme", t);
      }
    }

    // Read the parent's current theme where same-origin access allows it.
    try {
      const t = window.parent?.document?.documentElement?.getAttribute("data-theme");
      if (t) applyTheme(t);
    } catch (e) {
      /* cross-origin: wait for the postMessage instead */
    }

    const onMessage = (e) => {
      if (e.data?.type === "THEME_CHANGE") applyTheme(e.data.theme);
    };
    window.addEventListener("message", onMessage);
    return () => window.removeEventListener("message", onMessage);
  });

  $effect(() => () => {
    cancelAnimationFrame(tickHandle);
    worker?.terminate();
  });

  let accPct = $derived(accuracy === null ? "—" : `${(accuracy * 100).toFixed(1)}%`);
  let lossTxt = $derived(loss === null ? "—" : loss.toFixed(4));
</script>

<div class="page">
  <header class="hdr">
    <div>
      <p class="eyebrow">Svelte · Python · WebAssembly</p>
      <h1>A tiny neural network, trained in your browser</h1>
      <p class="sub">
        Backpropagation written by hand in Python — no autograd, no numpy, no server.
        It runs on Pyodide in a Web Worker; the page just draws what it reports.
      </p>
    </div>

    <dl class="stats">
      <div><dt>Epoch</dt><dd>{epoch}</dd></div>
      <div><dt>Loss</dt><dd>{lossTxt}</dd></div>
      <div><dt>Accuracy</dt><dd class="acc">{accPct}</dd></div>
    </dl>
  </header>

  <div class="grid">
    <!-- ---------------- controls ---------------- -->
    <aside class="panel controls">
      <h2 class="ph">Dataset</h2>
      <div class="segmented">
        {#each DATASETS as d}
          <button
            class:on={dataset === d.id}
            onclick={() => (dataset = d.id)}
            disabled={phase !== "ready"}
          >{d.label}</button>
        {/each}
      </div>

      <h2 class="ph">Architecture</h2>
      <label class="field">
        <span>Hidden layers <em>(comma separated, max 3)</em></span>
        <input
          type="text"
          bind:value={hiddenSpec}
          disabled={phase !== "ready"}
          spellcheck="false"
        />
      </label>

      <label class="field">
        <span>Activation</span>
        <select bind:value={activation} disabled={phase !== "ready"}>
          {#each ACTIVATIONS as a}<option value={a}>{a}</option>{/each}
        </select>
      </label>

      <h2 class="ph">Training</h2>
      <label class="field">
        <span>Learning rate <b>{Number(lr).toFixed(2)}</b></span>
        <input type="range" min="0.05" max="1.5" step="0.05" bind:value={lr} disabled={phase !== "ready"} />
      </label>

      <label class="field">
        <span>Data noise <b>{Number(noise).toFixed(2)}</b></span>
        <input type="range" min="0" max="0.4" step="0.01" bind:value={noise} disabled={phase !== "ready"} />
      </label>

      <div class="actions">
        <button class="primary" onclick={toggleRun} disabled={phase !== "ready"}>
          {#if running}
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="6" y="5" width="4" height="14" rx="1"/><rect x="14" y="5" width="4" height="14" rx="1"/></svg>
            Pause
          {:else}
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5l11 7-11 7z"/></svg>
            Train
          {/if}
        </button>
        <button onclick={() => step(false)} disabled={phase !== "ready" || running}>Step</button>
        <button onclick={rebuild} disabled={phase !== "ready"} title="Re-initialise the weights">Reset</button>
      </div>

      {#if phase === "ready"}
        <p class="foot">
          CPython {pyVersion} · {EPOCHS_PER_TICK} epochs/frame
          {#if lastMs}· {lastMs.toFixed(0)} ms{/if}
        </p>
      {/if}
    </aside>

    <!-- ---------------- visualisation ---------------- -->
    <section class="viz">
      <div class="board">
        <BoundaryCanvas {grid} {points} res={GRID_RES} extent={2.0} />

        {#if phase !== "ready"}
          <div class="veil">
            {#if phase === "idle"}
              <svg class="spark" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M13 2L4.5 13.5H11l-1 8.5 8.5-11.5H12l1-8.5z" />
              </svg>
              <h3>Start the Python engine</h3>
              <p>
                Downloads the CPython runtime (≈5&nbsp;MB, once) from a public CDN.
                Nothing is sent to a server — training happens on your machine.
              </p>
              <button class="primary big" onclick={startRuntime}>Start Python engine</button>
            {:else if phase === "loading"}
              <div class="spinner" aria-hidden="true"></div>
              <h3>Booting CPython</h3>
              <p class="mono">{statusLine}</p>
            {:else if phase === "error"}
              <h3 class="bad">Could not start the runtime</h3>
              <p class="mono">{errorMsg}</p>
              <button class="primary" onclick={() => { phase = "idle"; }}>Try again</button>
            {/if}
          </div>
        {/if}
      </div>

      <div class="side">
        <NetworkDiagram {topology} training={running} />
        <LossChart {history} />
      </div>
    </section>
  </div>

  <!-- ---------------- source ---------------- -->
  <section class="panel code">
    <button class="disclosure" onclick={() => (showCode = !showCode)} aria-expanded={showCode}>
      <svg class:open={showCode} viewBox="0 0 24 24" aria-hidden="true">
        <path d="M9 6l6 6-6 6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span>trainer.py</span>
      <em>the exact source running above</em>
    </button>

    {#if showCode}
      <pre><code>{trainerSource}</code></pre>
    {/if}
  </section>
</div>

<style>
  .page {
    max-width: 1120px;
    margin: 0 auto;
    padding: 1.75rem 1.25rem 3rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  /* ---- header ---- */
  .hdr {
    display: flex;
    flex-wrap: wrap;
    gap: 1.5rem;
    align-items: flex-end;
    justify-content: space-between;
  }

  .eyebrow {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.64rem;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: color-mix(in oklab, var(--content) 52%, transparent);
    margin: 0 0 0.5rem;
  }

  h1 {
    font-size: clamp(1.5rem, 3.4vw, 2.15rem);
    line-height: 1.12;
    max-width: 20ch;
  }

  .sub {
    margin: 0.7rem 0 0;
    max-width: 56ch;
    font-size: 0.9rem;
    line-height: 1.6;
    color: color-mix(in oklab, var(--content) 72%, transparent);
  }

  .stats {
    display: flex;
    gap: 1.75rem;
    margin: 0;
  }

  .stats div {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }

  .stats dt {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.6rem;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: color-mix(in oklab, var(--content) 50%, transparent);
  }

  .stats dd {
    margin: 0;
    font-size: 1.35rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
  }

  .stats .acc {
    color: var(--success);
  }

  /* ---- layout ---- */
  .grid {
    display: grid;
    grid-template-columns: 270px 1fr;
    gap: 1.1rem;
    align-items: start;
  }

  .viz {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 1.1rem;
    align-items: start;
  }

  .side {
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
  }

  @media (max-width: 1000px) {
    .grid {
      grid-template-columns: 1fr;
    }
    .viz {
      grid-template-columns: 1fr;
    }
  }

  .panel {
    background: var(--surface);
    border: 1px solid var(--hairline);
    border-radius: 14px;
    backdrop-filter: blur(14px) saturate(150%);
    -webkit-backdrop-filter: blur(14px) saturate(150%);
    box-shadow: var(--elev);
  }

  .controls {
    padding: 1.1rem;
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    position: sticky;
    top: 1rem;
  }

  .ph {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.6rem;
    letter-spacing: 0.16em;
    text-transform: uppercase;
    color: color-mix(in oklab, var(--content) 52%, transparent);
    margin: 0.5rem 0 0.15rem;
  }
  .ph:first-child {
    margin-top: 0;
  }

  /* ---- controls ---- */
  .segmented {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.35rem;
  }

  .segmented button {
    padding: 0.45rem 0.5rem;
    font-size: 0.78rem;
    font-weight: 600;
    border-radius: 8px;
    border: 1px solid var(--hairline);
    background: color-mix(in oklab, var(--base-200) 55%, transparent);
    color: color-mix(in oklab, var(--content) 70%, transparent);
    cursor: pointer;
    transition: all 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .segmented button:hover:not(:disabled) {
    color: var(--content);
    transform: translateY(-1px);
  }

  .segmented button.on {
    background: var(--primary);
    border-color: var(--primary);
    color: var(--primary-content);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    font-size: 0.78rem;
  }

  .field > span {
    color: color-mix(in oklab, var(--content) 72%, transparent);
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 0.5rem;
  }

  .field em {
    font-style: normal;
    font-size: 0.66rem;
    opacity: 0.6;
  }

  .field b {
    font-family: "JetBrains Mono", monospace;
    font-variant-numeric: tabular-nums;
    color: var(--primary);
  }

  input[type="text"],
  select {
    width: 100%;
    padding: 0.4rem 0.55rem;
    border-radius: 8px;
    border: 1px solid var(--hairline-strong);
    background: var(--base-100);
    color: var(--content);
    font-family: "JetBrains Mono", monospace;
    font-size: 0.8rem;
  }

  input[type="range"] {
    width: 100%;
    accent-color: var(--primary);
  }

  .actions {
    display: flex;
    gap: 0.4rem;
    margin-top: 0.75rem;
  }

  .actions button {
    flex: 1;
    padding: 0.55rem 0.5rem;
    font-size: 0.8rem;
    font-weight: 700;
    border-radius: 9px;
    border: 1px solid var(--hairline-strong);
    background: color-mix(in oklab, var(--base-200) 60%, transparent);
    color: var(--content);
    cursor: pointer;
    transition: all 0.28s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .actions button:hover:not(:disabled) {
    transform: translateY(-2px);
  }

  button.primary {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    background: var(--primary);
    border-color: var(--primary);
    color: var(--primary-content);
  }

  button.primary svg {
    width: 15px;
    height: 15px;
    fill: currentColor;
  }

  button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .foot {
    margin: 0.5rem 0 0;
    font-family: "JetBrains Mono", monospace;
    font-size: 0.62rem;
    color: color-mix(in oklab, var(--content) 45%, transparent);
  }

  /* ---- board + veil ---- */
  .board {
    position: relative;
  }

  .veil {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.6rem;
    text-align: center;
    padding: 1.5rem;
    border-radius: 12px;
    background: color-mix(in oklab, var(--base-100) 78%, transparent);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border: 1px solid var(--hairline);
  }

  .veil h3 {
    font-size: 1.05rem;
  }

  .veil p {
    margin: 0;
    max-width: 34ch;
    font-size: 0.8rem;
    line-height: 1.55;
    color: color-mix(in oklab, var(--content) 68%, transparent);
  }

  .veil .mono {
    font-family: "JetBrains Mono", monospace;
    font-size: 0.72rem;
  }

  .veil .bad {
    color: oklch(60% 0.19 25);
  }

  .veil button.big {
    margin-top: 0.5rem;
    padding: 0.65rem 1.2rem;
    font-size: 0.85rem;
    font-weight: 700;
    border-radius: 10px;
    border: 1px solid var(--primary);
    cursor: pointer;
  }

  .spark {
    width: 30px;
    height: 30px;
    fill: var(--accent);
    animation: bob 2.6s ease-in-out infinite;
  }

  @keyframes bob {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-5px); }
  }

  .spinner {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 2px solid color-mix(in oklab, var(--primary) 25%, transparent);
    border-top-color: var(--primary);
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* ---- code ---- */
  .code {
    overflow: hidden;
  }

  .disclosure {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.55rem;
    padding: 0.8rem 1rem;
    background: none;
    border: none;
    color: var(--content);
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 700;
    font-family: "JetBrains Mono", monospace;
    text-align: left;
  }

  .disclosure svg {
    width: 15px;
    height: 15px;
    flex-shrink: 0;
    transition: transform 0.3s cubic-bezier(0.22, 1, 0.36, 1);
  }

  .disclosure svg.open {
    transform: rotate(90deg);
  }

  .disclosure em {
    font-style: normal;
    font-weight: 400;
    font-family: "Manrope", sans-serif;
    font-size: 0.75rem;
    color: color-mix(in oklab, var(--content) 50%, transparent);
  }

  pre {
    margin: 0;
    padding: 0 1rem 1rem;
    max-height: 460px;
    overflow: auto;
    font-size: 0.72rem;
    line-height: 1.65;
    color: color-mix(in oklab, var(--content) 85%, transparent);
  }
</style>
